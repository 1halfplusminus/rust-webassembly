# Prototype Game Engine

## wasm-pack-template

**A template for kick starting A template for kick starting a Rust and WebAssembly project using**.

[wasm-pack](https://github.com/rustwasm/wasm-pack)

[Tutorial]

Built with 🦀🕸 by [RustWasm] The Rust and WebAssembly Working Group

[Tutorial]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

[RustWasm]: https://rustwasm.github.io/

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```sh
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### Rust's

```sh

rustup toolchain install nightly
rustup default nightly
rustup override set nightly
rustup target add wasm32-unknown-unknown

```

### 🛠️ Build with `wasm-pack build`

```sh
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```sh
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```sh
wasm-pack publish
```

## Android

[Android Doc](https://github.com/rust-windowing/android-ndk-rs)

### Install sdk

```sh

sudo apt install android-sdk google-android-ndk-installer
export ANDROID_SDK_ROOT="/usr/lib/android-sdk"
export ANDROID_NDK_ROOT="/usr/lib/android-ndk"
export PATH="${PATH}:${ANDROID_SDK_ROOT}/tools/:${ANDROID_SDK_ROOT}/platform-tools/"
```

### Build

#### build image

```sh

docker build . -t aarch64-linux-android:latest -f ./android.Dockerfile

```

```sh
RUST_BACKTRACE=1 cross build --target aarch64-linux-android
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
