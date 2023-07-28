# Synopsis

This project serves as an example/template project for building an application using WebAssembly Component Model (and in this case, an app for Ziverge's Golem Cloud).  It is my recommendation on how to structure such a project.

## Problem statement

The [`cargo-component` project](https://github.com/bytecodealliance/cargo-component) by Bytecode Alliance greatly smooths out the development process in building WebAssembly Component Model applications (in Rust).  You will define your app's data structures (records and enums) and function interfaces in a [`wit` (Wasm interface type)](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) file, which is then used to generate the Rust bindings for your Wasm component.  The generated code sits in a module somewhere inside the `target` directory.  Running `cargo component build` will build a valid program successfully.  However, unfortunately as of this writing, the regular Rust tooling (e.g. `rust-analyzer`) does not have visibility to this module.  As a result, we will most likely see red squiggly lines in our IDE for any data types defined in the `wit` file. And more importantly, `cargo test` will fail because it cannot resolve those references.

This is the motivation of creating a project structure that allows us to follow the development process we have accustomed to: write and run tests locally as well as running them on CI.

Although a discussion on the benefits (and the associated costs) of writing (and maintaining) tests is out of scope here, the ability to test our apps locally in this case can be quite desireable.  Even though it's feasible to test and debug our apps directly on the cloud, the feedback loop is much slower, let alone the time wasted on rebuilding and redeploying apps.  This project template makes it possible to run tests without having to change or disable regions of our Wasm component code just for the purpose of running tests.  It's my current recommendation on building WASI apps until `cargo-component` improves (or better alternatives become available).

Alright, let's dive in and I'll walk through the project structure.

## Workspace structure

### 1. Core module

The core business logic of our application goes into the `lib` module, where our code can be organized into logical units and only a select set of functions (and structs) are exposed as public APIs.

Just as any Rust apps, we can write unit tests in each sub-module as well as integration tests in a separate `tests` module.

To run tests at the root of our project, simply do `cargo nextest run -p lib`. (I highly recommend using `cargo-nextest` as the test runner.)  Note the `-p` parameter at the end of the command--we are passing the name of this module as the value.

We can also check test coverage by running `cargo tarpaulin -p lib`. (Run `cargo install cargo-tarpaulin` to add the sub-command.)

### 2. Console app

This one is optional.  However, I feel it's beneficial to build a console app, which not only provides us a way to test the `lib` module but, rather importantly, it can guide us through the process of designing our APIs, especially in the early stages.  Even though calling the `lib` module in a console app might be slightly different than calling it from the Wasm component--because the program flow will be likely different--we can still try to mimic the cloud API flow (which the Wasm component will expose--we're speaking in the context of running it on Golem Cloud here) as much as possible.  By doing so, this console app shall give us a very close feel of how our API will work.  This will provide a fast feedback loop and allow us to iterate quicker and with more confidence; at the same time it should help minimize the potential of a bad API design.

To run the console app at the root of the project directory, do `cargo run -p app`. Again, we will pass the `-p` parameter and specify the `app` module.  If you actually want to produce a binary of the console app, running `cargo build -p app` will produce the executable as `target/debug/app` (or `target/release/app` if `--release` is also passed to the build command).

### 3. Golem Cloud app

Now that we have tests and the console app to guide the implementation, at this point we should have a very good idea of how our APIs will look like.  Thereby we shall express our APIs in the `wit` file.  Next we will add some boilerplate code in the `wasm` module to glue the Rust bindings to our Wasm implementation. Please refer to the code in `wasm\lib.rs` as well as Golem's documentation.  The implementation in the Wasm module should be fairly trivial and quite similar to that of the console app.

To produce the Wasm binary, run `cargo component build --release -p wasm` at the root of our project directory.  This will produce the `target/wasm32-wasi/release/fib.wasm` file in this case.

## Running your app on Golem Cloud

For quick reference, here are the steps to run this app on Golem Cloud.  For details of various commands, please refer to Golem's documentation as well as Golem CLI's help.

Upload this app and run it on Golem Cloud:

1. Download Golem CLI from https://release.api.golem.cloud/ziverge/golem-cli.
2. Unzip the bundle to a directory.
3. Define a shell alias to the Golem CLI for convenience. For example:

  ```bash
  $ alias golem='{path-to-directory}/golem-cli/bin/golem'
  ```

4. Run `golem account get` to go through the authorization process if you haven't done so.
5. `cd` back to our project directory.
6. Run the following command to upload the app.

  ```bash
  $ golem component add --component-name fib target/wasm32-wasi/release/fib.wasm
  ```

7. Then run this command to create an instance of our app.

  ```bash
  $ golem instance add --instance-name fib-instance-1 --component-name fib
  ```

8. Define another shell alias to invoke the instance. For example:

  ```bash
  $ alias fib='golem instance invoke-and-await --instance-name fib-instance-1 --component-name fib --function $*'
  ```

> Note: `invoke-and-await` is akin to Akka's `ask` pattern whereas the `invoke` command is fire-and-forget.

9. Now let's run our app at last! 🎉

  * Run the `next` command to get the next Fibonacci number. Repeat it a few times to see if it produces the correct Fibonacci sequence.

  ```bash
  $ fib golem:it/api/next --parameters '[]'
  ```

  * Run the `reset` command to start over from 0.

  ```bash
  $ fib golem:it/api/reset --parameters '[]'
  ```

Congratulations! We have written, deployed and executed our first Golem app!

### Bonus

Now that we know our app is running as expected, we can test out the promise of Golem Cloud that apps are resilient and their states are preserved between interruptions.

We can interrupt our app by executing:

  ```bash
  $ golem instance interrupt --instance-name fib-instance-1 --component-name fib
  ```

 (or simulate a crash with `golem instance simulated-crash --instance-name fib-instance-1 --component-name fib`).

 After that, run `fib golem:it/api/next --parameters '[]'` again and verify that the Fibonacci number continues from that of the last invocation.

### Homework

I can tell you there is a bug somewhere in this implementation.  Can you identify it as an exercise?  If so, give it a shot at fixing it.  PRs are welcome! 🙂 I have something in mind for the fix (I actually have a reason why I don't want to fix it 😝) but I'm interested in other approaches.

Thanks for reading 🙏 and have fun writing Golem apps!
