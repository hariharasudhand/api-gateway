use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use env_logger::Builder;
use libloading::{Library, Symbol};
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct Policy {
    name: String,
    #[serde(rename = "in")]
    in_handlers: Vec<HandlerConfig>,
    #[serde(rename = "out")]
    out_handlers: Vec<HandlerConfig>,
    target: String,
    endpoint: String,
}

#[derive(Debug, Deserialize)]
struct HandlerConfig {
    name: String,
    params: HashMap<String, String>,
}

// Function to load policies from JSON file
fn load_policies() -> Vec<Policy> {
    let contents =
        fs::read_to_string("policy/policies.json").expect("Failed to read policies.json");
    serde_json::from_str(&contents).expect("Failed to parse policies.json")
}

// Function to execute a handler dynamically
fn execute_handler(handler_name: &str, params: &HashMap<String, String>) {
    unsafe {
        let library_path = "/Users/harid/work/rust-play/api-gateway/handler_library/target/release/libhandler_library.dylib";
        let lib = Library::new(library_path).expect("Failed to load library");

        // Construct function name based on handler_name
        let func_name = format!("execute_{}", handler_name);

        // Load the symbol from the library
        let func: Symbol<unsafe extern "C" fn(&HashMap<String, String>)> =
            lib.get(func_name.as_bytes()).expect("Symbol not found");

        // Call the function
        func(params);
    }
}

// Handler function for executing policies based on URL path
async fn execute_policy(req: HttpRequest, policies: web::Data<Arc<Vec<Policy>>>, client: web::Data<Client>) -> HttpResponse {
    // Extract policy name from the URL path
    let policy_name = req.match_info().get("policy_name").unwrap_or("default_policy");

    // Find the policy with the matching name
    if let Some(policy) = policies.iter().find(|p| p.name == policy_name) {
        println!("Applying Policy: {}", policy.name);

        // Execute inbound handlers
        process_handlers(&policy.in_handlers);

        // Call the target URL (assuming it's a placeholder)
        let url = format!("{}/{}", policy.target, policy.endpoint);
        println!("Calling Target URL: {}", url);

        // Make the HTTP request
        let response = client.get(&url).send().await;

        match response {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_else(|_| "Failed to read response body".into());

                // Execute outbound handlers
                process_handlers(&policy.out_handlers);

                // Return the response
                HttpResponse::build(status).body(body)
            }
            Err(err) => {
                // Return an error response if the request fails
                HttpResponse::InternalServerError().body(format!("Failed to call target URL: {}", err))
            }
        }
    } else {
        // Return a 404 Not Found response if policy not found
        HttpResponse::NotFound().body(format!("Policy '{}' not found", policy_name))
    }
}

// Function to process handlers
fn process_handlers(handlers: &[HandlerConfig]) {
    for handler in handlers {
        println!("  Handler Name: {}", handler.name);
        println!("  Handler Params: {:?}", handler.params);

        // Execute handler based on its name
        execute_handler(&handler.name, &handler.params);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure logging
    Builder::new().filter_level(log::LevelFilter::Debug).init();

    // Load policies from JSON file
    let policies = load_policies();
    println!("Loaded Policies: {:#?}", policies);

    // Convert policies to Arc<Vec<Policy>>
    let policies_arc = Arc::new(policies);

    // Print the routes configuration
    println!("Registered routes:");
    println!("GET /{{policy_name}}");

    // Create an HTTP client
    let client = Client::new();

    // Start Actix Web server
    HttpServer::new(move || {
        // Clone Arc reference for each thread
        let policies_ref = policies_arc.clone();

        App::new()
            .app_data(web::Data::new(policies_ref.clone())) // Share policies across threads
            .app_data(web::Data::new(client.clone())) // Share the HTTP client across threads
            .route("/{policy_name}", web::get().to(execute_policy))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
