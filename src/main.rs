extern crate wasmer_runtime;

use std::sync::{Arc, Mutex};
use wasmer_runtime::{error, func, imports, instantiate, Array, Ctx, WasmPtr};

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] =
    include_bytes!("../wasm-sample-app/target/wasm32-unknown-unknown/release/wasm_sample_app.wasm");

fn main() -> error::Result<()> {
    // create shared data that we'll use in 2 host functions
    let shared_data = Arc::new(Mutex::new(0usize));

    // copy the [`Arc`] and move it into the closure
    let data = Arc::clone(&shared_data);
    let print_str2 = move |ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32| {
        let memory = ctx.memory(0);

        // Use helper method on `WasmPtr` to read a utf8 string
        let string = ptr.get_utf8_string(memory, len).unwrap();

        // Get the value from the shared data
        let guard = data.lock().unwrap();
        // Print it!
        println!("{}: {}", guard, string);
    };

    // Copy the [`Arc`] and move it into the closure
    let data = Arc::clone(&shared_data);
    let increment_shared = move || {
        // get the shared data and increment it
        let mut guard = data.lock().unwrap();
        *guard += 1;
    };
    // Let's define the import object used to import our function
    // into our webassembly sample application.
    //
    // We've defined a macro that makes it super easy.
    //
    // The signature tells the runtime what the signature (the parameter
    // and return types) of the function we're defining here is.
    // The allowed types are `i32`, `u32`, `i64`, `u64`,
    // `f32`, and `f64`.
    //
    // Make sure to check this carefully!
    let import_object = imports! {
        // Define the "env" namespace that was implicitly used
        // by our sample application.
        "env" => {
            // name        // the func! macro autodetects the signature
            "print_str" => func!(print_str),
            // we can use closures here too
            "print_str2" => func!(print_str2),
            "increment_shared" => func!(increment_shared),
        },
    };

    // Compile our webassembly into an `Instance`.
    let instance = instantiate(WASM, &import_object)?;

    // Call our exported function!
    instance.call("hello_wasm", &[])?;

    Ok(())
}

// Let's define our "print_str" function.
//
// The declaration must start with "extern" or "extern "C"".
fn print_str(ctx: &mut Ctx, ptr: WasmPtr<u8, Array>, len: u32) {
    // Get a slice that maps to the memory currently used by the webassembly
    // instance.
    //
    // Webassembly only supports a single memory for now,
    // but in the near future, it'll support multiple.
    //
    // Therefore, we don't assume you always just want to access first
    // memory and force you to specify the first memory.
    let memory = ctx.memory(0);

    // Use helper method on `WasmPtr` to read a utf8 string
    let string = ptr.get_utf8_string(memory, len).unwrap();

    // Print it!
    println!("{}", string);
}
