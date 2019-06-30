# rust-to-ios
[![Swift 5.0](https://img.shields.io/badge/swift-5.0-orange.svg?style=flat)](#)
[![@wojteklu](https://img.shields.io/badge/contact-@wojteklu-blue.svg?style=flat)](https://twitter.com/wojteklu)

Example project for building a library for iOS.

* ✓ sending primitives between Rust and iOS
* ✓ sending strings between Rust and iOS
* ✓ giving ownership of a Rust instance to Swift

Setup
-----

1. Download [rustup](https://www.rustup.rs/) needed to setup Rust for cross-compiling.

    ```sh
    curl https://sh.rustup.rs -sSf | sh
    ```

2. Download targets for iOS.

    ```sh
    rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
    ```

3. Install cargo-lipo to generate the iOS universal library.

    ```sh
    cargo install cargo-lipo
    ```

Building the library
--------------------

1. Create a new cargo project.

    ```sh
    cargo new message
    ```

2. Update Cargo.toml by adding the [lib] section.

    ```sh
    [lib]
    name = "message"
    crate-type = ["staticlib"]
    ```

3. Write the library and [expose its public interface in a C header](https://doc.rust-lang.org/book/first-edition/ffi.html).

4. Build the library.

    ```sh
    cd message

    cargo lipo --release
    ```

Using the library
-----------------

1. Create the iOS project.

2. Add the C header to allow using the Rust functions from C.

3. Copy `target/universal/release/libmessage.a` to project.

4. Add `libresolv.tbd` to `Linked frameworks and libraries`.

5. Note that `cargo-lipo` does not support bitcode yet. You must set `ENABLE_BITCODE` to `NO`.

## Author

Wojtek Lukaszuk [@wojteklu](http://twitter.com/wojteklu)

## License

Available under the MIT license. See the LICENSE file for more info.
