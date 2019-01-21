
// Defines functions to import from "env" namespace
extern "C" {
    fn print_str(ptr: *const u8, len: usize);
}

#[no_mangle]
pub extern fn hello_wasm(){
    let message = "Hello World";
    unsafe {
      print_str(message.as_ptr(), message.len());
    }
}