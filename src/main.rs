extern crate wasmer_clif_backend;
extern crate wasmer_runtime;

use std::{fs::File, io::prelude::*, str};

use wasmer_clif_backend::CraneliftCompiler;
use wasmer_runtime::{self as runtime, prelude::*};

fn main() {
    // Read the wasm file produced by our sample application...
    let mut wasm_file =
        File::open("./wasm-sample-app/target/wasm32-unknown-unknown/release/wasm_sample_app.wasm")
            .unwrap();
    // ... and put it into a vector.
    let mut wasm_bytes = Vec::new();
    wasm_file.read_to_end(&mut wasm_bytes).unwrap();

    // Instantiate the compiler we're going to use. The wasmer-runtime
    // is designed to support multiple compiler backends. Right now,
    // only the Cranelift compiler is supported, but we're working on
    // an LLVM backend as well!
    let compiler = CraneliftCompiler::new();

    // Compile our webassembly into a wasmer-runtime `Module`.
    let module = runtime::compile(&wasm_bytes, &compiler).unwrap();

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
            // name         // func    // signature
            "print_str" => print_str<[u32, u32] -> []>,
        },
    };

    // Here we go!
    //
    // Instantiate the module with the imports we just created
    // to create, you guessed it, an `Instance`.
    //
    // You can create any number of instances with a single module.
    let mut instance = module.instantiate(import_object).unwrap();

    // At last, we can call the function exported by our webassembly
    // sample application.
    //
    // Since our exported function doesn't receive any parameters,
    // we just pass it an empty slice as the parameter list.
    instance.call("hello_wasm", &[]).unwrap();
}

// Let's define our "print_str" function.
//
// The declaration must start with "extern" or "extern "C"".
extern "C" fn print_str(ptr: u32, len: u32, vmctx: &mut vm::Ctx) {
    // Get a slice that maps to the memory currently used by the webassembly
    // instance.
    //
    // Webassembly only supports a single memory for now,
    // but in the near future, it'll support multiple.
    //
    // Therefore, we don't assume you always just want to access first
    // memory and force you to specify the first memory.
    let memory = vmctx.memory(0);

    // Get a subslice that corresponds to the memory used by the string.
    let str_slice = &memory[ptr as usize..(ptr + len) as usize];

    // Convert the subslice to a `&str`.
    let str = str::from_utf8(str_slice).unwrap();

    // Print it!
    println!("{}", str);
}
