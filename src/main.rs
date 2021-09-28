use wasmer::{imports, wat2wasm, Function, Instance, Module, NativeFunc, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

mod admin;
mod router;
mod s_env;

// todo thorough testing

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     s_env::init_logging();
//
//     let db = s_env::RockWrapper::init("rock");
//     let env = s_env::validate_env();
//
//     HttpServer::new(move || {
//         App::new()
//             .data(db.clone())
//             .data(env.clone())
//             .configure(router::router)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// https://rustwasm.github.io/docs/wasm-bindgen/examples/import-js.html
// https://github.com/wasmerio/wasmer/blob/master/examples/README.md

fn main() {
    let wasm_bytes = include_bytes!("/home/mark/CLionProjects/wasm_test/pkg/wasm_test_bg.wasm");

    // Next we create the `Store`, the top level type in the Wasmer API.
    //
    // Note that we don't need to specify the engine/compiler if we want to use
    // the default provided by Wasmer.
    // You can use `Store::default()` for that.
    //
    // However for the purposes of showing what's happening, we create a compiler
    // (`Cranelift`) and pass it to an engine (`Universal`). We then pass the engine to
    // the store and are now ready to compile and run WebAssembly!
    let store = Store::new(&Universal::new(Cranelift::default()).engine());

    // We then use our store and Wasm bytes to compile a `Module`.
    // A `Module` is a compiled WebAssembly module that isn't ready to execute yet.
    let module = Module::new(&store, wasm_bytes).unwrap();

    // Next we'll set up our `Module` so that we can execute it.

    // We define a function to act as our "env" "say_hello" function imported in the
    // Wasm program above.
    fn say_hello_world() {
        println!("Hello, world!")
    }

    // We then create an import object so that the `Module`'s imports can be satisfied.
    let import_object = imports! {
        // We use the default namespace "env".
        "env" => {
            // And call our function "say_hello".
            "name" => Function::new_native(&store, say_hello_world),
        }
    };

    // We then use the `Module` and the import object to create an `Instance`.
    //
    // An `Instance` is a compiled WebAssembly module that has been set up
    // and is ready to execute.
    let instance = Instance::new(&module, &import_object).unwrap();

    // We get the `NativeFunc` with no parameters and no results from the instance.
    //
    // Recall that the Wasm module exported a function named "run", this is getting
    // that exported function from the `Instance`.
    let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("hello_world").unwrap();

    // Finally, we call our exported Wasm function which will call our "say_hello"
    // function and return.
    run_func.call().unwrap();
}
