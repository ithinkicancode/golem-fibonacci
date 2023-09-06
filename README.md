# Synopsis

This project serves as an example/template for building an application using [WebAssembly Component Model](https://github.com/webassembly/component-model) (and in this case, an app for Ziverge's [Golem Cloud](https://www.golem.cloud/post/unveiling-golem-cloud)).  It is my recommendation on how to structure such a project.

Before I start the walkthrough and explain the project structure, make sure you have set up Rust's toolchain and installed `cargo-component`. Please refer to [Golem Cloud documentation](https://www.golem.cloud/learn/rust) for instructions.

## Workspace structure

### 1. Core module

The core business logic of our application goes into the `lib` module, where our code can be organized into logical units and only a select set of functions (and structs) are exposed as public APIs. This is quite common in Rust applications.

Like most Rust apps, we can write unit tests in each sub-module as well as integration tests in a separate `tests` module.

To run tests at the project's root, simply do `cargo nextest run -p lib`. (I highly recommend using `cargo-nextest` as the test runner.)  Note the `-p` parameter at the end of the command -- we are passing the name of this module as the value.

We can also check test coverage by running `cargo tarpaulin -p lib`. (Run `cargo install cargo-tarpaulin` to add the sub-command.)

### 2. Console app

This one is optional.  However, I feel it's beneficial to build a console app, which not only provides us a way to system-test our APIs defined in the `lib` module but, rather importantly, it can guide us through the process of designing the APIs, especially in the early iterations.  While calling the APIs in a console app might be slightly different than calling them from the Wasm component (because the execution flow may likely be different), we can still try to mimic, as much as possible, the cloud API flow (exposed by the Wasm component -- we're speaking in the context of running it on Golem Cloud here).  By doing so, this console app shall give us a very close feel of how our APIs will work.  This will provide a fast feedback loop and allow us to iterate quicker and with more confidence; at the same time it should help minimize the potential of a bad API design.

To run the console app at the project root, do `cargo run -p app`.  Again, we will pass the `-p` parameter and specify the `app` module.  If you want to produce a binary of the console app, running `cargo build -p app` will produce the executable as `target/debug/app` (or `target/release/app` if the `--release` flag is included in the build command).

### 3. Wasm app

Now that we have tests and the console app to guide the implementation, we should have a very good idea of how our APIs will look like.  Thereby, we shall express our APIs in the `wit` file.  Next, we will add some boilerplate code in the `wasm` module to glue the Rust bindings to our Wasm implementation. Please refer to the code in `wasm\lib.rs` as well as Golem's documentation.  The implementation in the Wasm module should be fairly trivial and quite similar to that of the console app.

To build the Wasm assembly, run `cargo component build --release -p wasm` at the root of our project directory.  This will produce the `target/wasm32-wasi/release/wasm.wasm` file in this case.

## Running the app on Golem Cloud

As a quick reference, here are the steps to run this app on Golem Cloud.  For details of various commands, please refer to [Golem CLI documentation](https://www.golem.cloud/learn/golem-cli) as well as its built-in help.

Upload this app and run it on Golem Cloud:

1. Download the latest version of Golem CLI by [signing up](https://www.golem.cloud/sign-up) for the Developer Preview.
2. Install Golem CLI by running `cargo install golem-cli`.
3. Run `golem-cli account get` to go through the authorization process if you haven't done so.
4. `cd` to the root of our project directory.
5. Run the following command to upload the binary.

  ```bash
  golem-cli template add --template-name fib target/wasm32-wasi/release/wasm.wasm
  ```

6. Next, run this command to create a worker for our app.

  ```bash
  golem-cli worker add --worker-name fib-wrkr-1 --template-name fib
  ```

7. Let's define a shell alias to invoke the instance. For example:

  ```bash
  alias fib='golem-cli worker invoke-and-await --worker-name fib-wrkr-1 --template-name fib --function $*'
  ```

> Note: `invoke-and-await` is akin to Akka's `ask` pattern whereas the `invoke` command is fire-and-forget.

8. At last let's run our app! 🎉

  * Run the `next` command to get the next Fibonacci number. Repeat it a few times to verify that it produces the correct Fibonacci sequence.

  ```bash
  fib golem:fib/api/next --parameters '[]'
  ```

  * Run the `reset` command to start over from 0.

  ```bash
  fib golem:fib/api/reset --parameters '[]'
  ```

Congratulations! We have written, deployed and executed our first Golem app!

### Bonus

Now that we know our app is running as expected, we can test out the promise of Golem Cloud that apps are resilient and their states are preserved between interruptions.

We can interrupt our app by executing:

  ```bash
  golem-cli worker interrupt --worker-name fib-wrkr-1 --template-name fib
  ```

 (or simulate a crash with `golem-cli worker simulated-crash --worker-name fib-wrkr-1 --template-name fib`).

 After that, run `fib golem:fib/api/next --parameters '[]'` again and verify that the Fibonacci number continues from that of the last invocation.

### Homework

I can tell you there is a bug somewhere in this implementation.  Can you identify it as an exercise?  If so, give it a shot at fixing it.  PRs are welcome! 🙂 I have something in mind for the fix (I actually have a reason why I don't want to fix it 😝) but I'm interested in other solutions.

Thanks for reading 🙏 and have fun writing Golem apps!

### P.S.

This project is definitely very very simple! For (slightly) more complex apps, check out my other Golem's projects [here](https://github.com/ithinkicancode/golem-wordle) and [here](https://github.com/ithinkicancode/golem-todo-list).
