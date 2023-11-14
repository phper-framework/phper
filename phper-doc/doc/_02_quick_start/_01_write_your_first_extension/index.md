# Write your first extension

Here we will write the `hello world` extension, which has a function, receive the person name and echo hello to the person.

Full example is <https://github.com/phper-framework/phper/tree/master/examples/hello>.

## Steps

1. Make sure `libclang` is installed (required by [bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html)).

   `phper` require libclang *9.0+*.

   ```shell
   # If you are using debian like linux system:
   sudo apt install llvm-10-dev libclang-10-dev
   ```

1. Create the cargo project, with the extension name.

   ```shell
   cargo new --lib hello

   cd hello
   ```

1. Add the metadata to the `Cargo.toml` to build the `.so` file.

   ```toml
   # Cargo.toml

   [lib]
   crate-type = ["cdylib"]
   ```

   Run the command to add `phper` dependency.

   ```shell
   cargo add phper
   ```

1. Create the `build.rs` (adapting MacOS).

   ```rust,no_run
   fn main() {
      #[cfg(target_os = "macos")]
      {
         println!("cargo:rustc-link-arg=-undefined");
         println!("cargo:rustc-link-arg=dynamic_lookup");
      }
   }
   ```

1. Add this code to `src/lib.rs`.

   ```rust
   use phper::{echo, functions::Argument, modules::Module, php_get_module, values::ZVal};
   
   /// The php function, receive arguments with type `ZVal`.
   fn say_hello(arguments: &mut [ZVal]) -> phper::Result<()> {
       // Get the first argument, expect the type `ZStr`, and convert to Rust utf-8
       // str.
       let name = arguments[0].expect_z_str()?.to_str()?;
   
       // Macro which do php internal `echo`.
       echo!("Hello, {}!\n", name);
   
       Ok(())
   }
   
   /// This is the entry of php extension, the attribute macro `php_get_module`
   /// will generate the `extern "C" fn`.
   #[php_get_module]
   pub fn get_module() -> Module {
       // New `Module` with extension info.
       let mut module = Module::new(
           env!("CARGO_PKG_NAME"),
           env!("CARGO_PKG_VERSION"),
           env!("CARGO_PKG_AUTHORS"),
       );
   
       // Register function `say_hello`, with one argument `name`.
       module.add_function("say_hello", say_hello).argument(Argument::by_val("name"));
   
       module
   }
   ```

1. Build, if your PHP isn't installed globally, you should specify the path of `php-config`.

   ```bash
   # Optional, specify if php isn't installed globally,
   # this environment is used by `phper-sys`.
   #
   # export PHP_CONFIG=<Your path of php-config>
   
   # Build libhello.so.
   cargo build
   ```

1. Run the php command with the extension.

   ```shell
   php -d "extension=target/debug/libhello.so" -r "say_hello('Bob');"
   ```

   Then you can get the output:

   ```text
   Hello, Bob!
   ```
