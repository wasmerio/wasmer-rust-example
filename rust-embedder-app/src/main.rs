extern crate wasmer_clif_backend;
extern crate wasmer_runtime;

use std::fs::File;
use std::io::prelude::*;
use std::str;

use wasmer_clif_backend::CraneliftCompiler;
use wasmer_runtime::{
    self as runtime,
    export::{Context, Export, FuncPointer},
    import::{Imports, NamespaceMap},
    structures::TypedIndex,
    types::{FuncSig, MemoryIndex, Type},
    vm,
};

fn main() {
    // Read the wasm bytes of our sample application
    let mut wasm_file =
        File::open("../wasm-sample-app/target/wasm32-unknown-unknown/release/wasm_sample_app.wasm")
            .unwrap();
    let mut wasm_bytes = Vec::new();
    wasm_file.read_to_end(&mut wasm_bytes).unwrap();

    // Compile a WebAssembly Module from wasm bytes
    let module = runtime::compile(&wasm_bytes, &CraneliftCompiler::new()).unwrap();

    // Imports is an import object provided to an instance during instantiation
    // to define its imports.
    let mut imports = Imports::new();

    // Define a top-level namespace "env" to hold our host function
    let mut env_namespace = NamespaceMap::new();
    env_namespace.insert(
        "print_str",
        Export::Function {
            func: unsafe { FuncPointer::new(print_str as _) },
            ctx: Context::Internal,
            signature: FuncSig {
                params: vec![Type::I32, Type::I32],
                returns: vec![],
            },
        },
    );
    // Register this namespace in the import object
    imports.register("env", env_namespace);

    // Create an instance of this module with the provided imports
    let mut instance = module.instantiate(imports).unwrap();

    // Call the instance's exported function
    instance.call("hello_wasm", &[]).unwrap();
}

extern "C" fn print_str(ptr: i32, len: i32, vmctx: *mut vm::Ctx) {
    // get the linear memory
    let memory = unsafe { (*vmctx).memory(MemoryIndex::new(0)) };

    // convert bytes to string
    let string = str::from_utf8(&memory[(ptr as usize)..((ptr + len) as usize)]).unwrap();
    println!("{}", string);
}
