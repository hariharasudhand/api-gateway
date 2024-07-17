**Api Gateway**

A Rust-based API Gateway

**Work in Progress**

This project is currently under development.

**Directory Structure**

_handler_library_: This directory contains Rust files for handlers. Handlers are categorized into two types: inbound (request handlers) and outbound (response handlers). A policy.json file is used to configure the target/endpoint and the inbound and outbound handlers.

_source:_ This directory contains the source code for the proxy server.
Running the Project
To run the project, navigate to the source directory and execute:


**cargo run**

This command will start the proxy server, which reads the policy.json file and prepares to serve requests. The server executes all inbound handlers, calls the target service, and then executes all outbound handlers on the response.

Note: Currently, only GET requests are supported. Future updates will include support for POST and other HTTP methods.

**Sample Policy.json**

''[
  {
    "name": "policy1",
    "in": [
      { "name": "handler1", "params": {} },
      { "name": "handler2", "params": {} }
    ],
    "out": [
      { "name": "handler1", "params": {} },
      { "name": "handler2", "params": {} }
    ],
    "target": "https://mocki.io/",
    "endpoint": "/v1/94105153-3aaa-49da-9959-e6ff5d1de56b"
  },
  {
    "name": "policy2",
    "in": [{ "name": "handler1", "params": {} }],
    "out": [
      { "name": "handler1", "params": {} },
      { "name": "handler2", "params": {} }
    ],
    "target": "http://www.google.com",
    "endpoint": "/service2"
  }
]
''
