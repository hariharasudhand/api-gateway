#[no_mangle]
pub extern "C" fn execute_handler2(params: *const std::ffi::c_void) {
    // Implementation of handler1
    println!("Handler 2 executed with params: {:?}", params);

}