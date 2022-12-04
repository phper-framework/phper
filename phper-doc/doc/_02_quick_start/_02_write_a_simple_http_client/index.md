# Write a simple http client

Here we will use Rust crate [reqwest](https://crates.io/crates/reqwest) to write a simple http client,
like curl, but object-oriented.

Full example is <https://github.com/phper-framework/phper/tree/master/examples/http-client>.

Imagine that our PHP API should look like this:

```php
<?php

/*** http-client.php ***/

use HttpClient\HttpClientBuilder;
use HttpClient\HttpClient;
use HttpClient\HttpClientException;

$client = (new HttpClientBuilder())
    ->timeout(15000)
    ->cookie_store(true)
    ->build();

$response = $client->get("https://httpbin.org/ip")->send();
var_dump([
    "status" => $response->status(),
    "headers" => $response->headers(),
    "body" => $response->body(),
]);
```

Here, the namespace of API is `HttpClient`.

And there are three class:

- `HttpClientBuilder` is the builder of `HttpClient`.
- `HttpClient` will send a http request and generate a http response.
- `HttpClientException` will be throw when http request failed.

## Steps

Before writing the code, we first prepare the dependency and startup code.

1. Make sure `libclang` is installed (required by [bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html)).

   `phper` require libclang *9.0+*.

   ```shell
   # If you are using debian like linux system:
   sudo apt install llvm-10-dev libclang-10-dev
   ```

1. Create the cargo project, with the extension name.

   ```shell
   cargo new --lib http-client

   cd http-client
   ```

1. Add the metadata to the `Cargo.toml` to build the `.so` file.

   ```toml
   ## Cargo.toml

   [lib]
   crate-type = ["cdylib"]
   ```

   Run the command to add dependencies.

   ```shell
   cargo add phper
   cargo add reqwest --features blocking --features cookies
   cargo add thiserror
   ```

1. Create the `build.rs` ( Adapting MacOS ).

   ```rust,no_run
   fn main() {
      #[cfg(target_os = "macos")]
      {
         println!("cargo:rustc-link-arg=-undefined");
         println!("cargo:rustc-link-arg=dynamic_lookup");
      }
   }
   ```

Now let's begin to finish the logic.

1. First, we create `src/errors.rs` to make the `HttpClientException` class:

   ```rust
   /*** src/errors.rs ***/

   use phper::classes::{ClassEntry, StatefulClass};
   
   /// The exception class name of extension.
   const EXCEPTION_CLASS_NAME: &str = "HttpClient\\HttpClientException";
   
   /// The struct implemented `phper::Throwable` will throw php Exception
   /// when return as `Err(e)` in extension functions.
   #[derive(Debug, thiserror::Error, phper::Throwable)]
   #[throwable_class(EXCEPTION_CLASS_NAME)]
   pub enum HttpClientError {
       /// Generally, implement `From` for `phper::Error`.
       #[error(transparent)]
       #[throwable(transparent)]
       Phper(#[from] phper::Error),
   
       #[error(transparent)]
       Reqwest(#[from] reqwest::Error),
   
       #[error("should call '{method_name}()' before call 'body()'")]
       ResponseAfterRead { method_name: String },
   
       #[error("should not call 'body()' multi time")]
       ResponseHadRead,
   }
   
   pub fn make_exception_class() -> StatefulClass<()> {
       let mut exception_class = StatefulClass::new(EXCEPTION_CLASS_NAME);
        // The `extends` is same as the PHP class `extends`.
       exception_class.extends("Exception");
       exception_class
   }
   ```

   > The `make_*_class` functions is for registering class in `src/lib.rs` later.

   > The `StatefulClass` represents the class entry hold the state as generic type,
   > so you can wrap the Rust struct as state in PHP class, which is the common usage
   > of class in php extensions (if using C/C++ to develop PHP extension, the PHP class
   > commonly wrap the C/C++ pointer).

   > But here the `HttpClientException` hasn't state required, so the class in
   > `StatefulClass<()>`.

1. Then, create the `HttpClientBuilder` class in `src/client.rs`.

   ```rust
   /*** src/errors.rs ***/

   use phper::classes::{ClassEntry, StatefulClass};
   
   /// The exception class name of extension.
   const EXCEPTION_CLASS_NAME: &str = "HttpClient\\HttpClientException";
   
   /// The struct implemented `phper::Throwable` will throw php Exception
   /// when return as `Err(e)` in extension functions.
   #[derive(Debug, thiserror::Error, phper::Throwable)]
   #[throwable_class(EXCEPTION_CLASS_NAME)]
   pub enum HttpClientError {
       /// Generally, implement `From` for `phper::Error`.
       #[error(transparent)]
       #[throwable(transparent)]
       Phper(#[from] phper::Error),
   
       #[error(transparent)]
       Reqwest(#[from] reqwest::Error),
   
       #[error("should call '{method_name}()' before call 'body()'")]
       ResponseAfterRead { method_name: String },
   
       #[error("should not call 'body()' multi time")]
       ResponseHadRead,
   }

   /*** src/client.rs ***/

   use phper::{
       alloc::ToRefOwned,
       classes::Visibility,
       functions::Argument,
   };
   use reqwest::blocking::{Client, ClientBuilder};
   use std::{mem::take, time::Duration};

   const HTTP_CLIENT_BUILDER_CLASS_NAME: &str = "HttpClient\\HttpClientBuilder";
   const HTTP_CLIENT_CLASS_NAME: &str = "HttpClient\\HttpClient";

   pub fn make_client_builder_class() -> StatefulClass<ClientBuilder> {
       // `new_with_default_state` means initialize the state of `ClientBuilder` as
       // `Default::default`.
       let mut class = StatefulClass::new_with_default_state(HTTP_CLIENT_BUILDER_CLASS_NAME);

       // Inner call the `ClientBuilder::timeout`.
       class.add_method(
           "timeout",
           Visibility::Public,
           |this, arguments| {
               let ms = arguments[0].expect_long()?;
               let state = this.as_mut_state();
               let builder: ClientBuilder = take(state);
               *state = builder.timeout(Duration::from_millis(ms as u64));
               Ok::<_, HttpClientError>(this.to_ref_owned())
           },
       ).argument(Argument::by_val("ms"));

       // Inner call the `ClientBuilder::cookie_store`.
       class.add_method(
           "cookie_store",
           Visibility::Public,
           |this, arguments| {
               let enable = arguments[0].expect_bool()?;
               let state = this.as_mut_state();
               let builder: ClientBuilder = take(state);
               *state = builder.cookie_store(enable);
               Ok::<_, HttpClientError>(this.to_ref_owned())
           },
       ).argument(Argument::by_val("enable"));

       // Inner call the `ClientBuilder::build`, and wrap the result `Client` in Object.
       class.add_method(
           "build",
           Visibility::Public,
           |this, _arguments| {
               let state = take(this.as_mut_state());
               let client = ClientBuilder::build(state)?;
               let mut object = ClassEntry::from_globals(HTTP_CLIENT_CLASS_NAME)?
                                .init_object()?;
               unsafe {
                   *object.as_mut_state() = Some(client);
               }
               Ok::<_, HttpClientError>(object)
           },
       );

       class
   }
   ```

1. Follow this method to complete `HttpClient`, `RequestBuilder` and `Response`, see full example for details.

1. Register all classes in `src/lib.rs`.

1. All codes are finished, so we can build the extension `.so`, and run the
   php script of the beginning of the tutorial with the extension.

   ```shell
   cargo build

   php -d "extension=target/debug/libhttp_client.so" http-client.php
   ```

   Here is the result I got:

   ```text
   array(3) {
   ["status"]=>
   int(200)
   ["headers"]=>
   array(7) {
       ["date"]=>
       array(1) {
       [0]=>
       string(29) "Sat, 03 Dec 2022 09:15:11 GMT"
       }
       ["content-type"]=>
       array(1) {
       [0]=>
       string(16) "application/json"
       }
       ["content-length"]=>
       array(1) {
       [0]=>
       string(2) "33"
       }
       ["connection"]=>
       array(1) {
       [0]=>
       string(10) "keep-alive"
       }
       ["server"]=>
       array(1) {
       [0]=>
       string(15) "gunicorn/19.9.0"
       }
       ["access-control-allow-origin"]=>
       array(1) {
       [0]=>
       string(1) "*"
       }
       ["access-control-allow-credentials"]=>
       array(1) {
       [0]=>
       string(4) "true"
       }
   }
   ["body"]=>
   string(33) "{
   "origin": "223.104.76.175"
   }
   "
   }
   ```
