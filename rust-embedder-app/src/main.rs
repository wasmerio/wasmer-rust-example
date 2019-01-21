extern crate wasmer_clif_backend;
extern crate wasmer_runtime;

use std::fs::File;
use std::io::prelude::*;
use std::slice;
use std::str;

use wasmer_clif_backend::CraneliftCompiler;
use wasmer_runtime::{
    self as runtime,
    backing::LocalBacking,
    error::Result,
    export::{Context, Export, FuncPointer},
    import::{Imports, NamespaceMap},
    structures::TypedIndex,
    types::{FuncSig, LocalMemoryIndex, Type, Value},
    vm,
};

fn main() {
    // Read the wasm bytes of our sample application
    let mut wasm_file =
        File::open("../wasm-sample-app/target/wasm32-unknown-unknown/release/wasm_sample_app.wasm")
            .unwrap();
    let mut wasm_bytes = Vec::new();
    wasm_file.read_to_end(&mut wasm_bytes);

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
    instance.call("hello_wasm", &[]);
}

extern "C" fn print_str(ptr: i32, len: i32, vmctx: *mut vm::Ctx) {
    //    // read the memory
    //    let str_ptr_start = (*(*vmctx).local_backing).memory(LocalMemoryIndex::new(0)).base().offset(ptr);
    //    let slice = unsafe { slice::from_raw_parts(str_ptr_start, len) };
    //    // convert bytes to string
    //    let string = str::from_utf8(&slice).unwrap();
    //    println!("{}", string);
    println!("TODO implement print_str");
}
