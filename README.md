# tv-app-poc-rust

Rust TV POC for Tech Talk, here is an example of how use Rust to have the
main logic an compile to web-assembly or native code (JNI) for use
with Andorid NDK.

This was a POC created with such great team member
[@almahirpm](https://github.com/almahirpm), Yaser, Arturo

# Requirements
 - Node.js v12 or higher
 - Rust ([rustup][rust-install])

# Installation

## Geneal

Install the dependencies...

```bash
npm install
rustup install stable && rustup default stable
```

Build the assets

```bash
npm run build
```

## Android

- Install [android studio][android-studio]
- Create an TV Emulator
- Import the project from `wasmandroid` folder

## Tizen
- Install [tizen studio][tize-studio] IDE or CLI
- Install [Emscripten SDK][emscripten-sdk]
- Import the project from `app` folder

## Run svelte app 

...then start webpack:

```bash
npm run dev
```

Navigate to [localhost:8080](http://localhost:8080).


## Bonus Android Native + Rust!!
- Install [android NDK][android-ndk]
- Import the project from `rustandroid` folder
- Create an TV Emulator
- Add X86 target and build JNI lib

```bash
rustup target add i686-linux-android
npm run build:jni
```
- Run the emulator and see the magic! rust code as native android code!

 [tize-studio]: https://developer.samsung.com/smarttv/develop/getting-started/setting-up-sdk/installing-tv-sdk.html
 [emscripten-sdk]: https://developer.samsung.com/smarttv/develop/extension-libraries/webassembly/getting-started/downloading-and-installing.html
 [android-studio]: https://developer.android.com/studio/
 [rust-install]: https://www.rust-lang.org/tools/install
 [android-ndk]: https://developer.android.com/ndk
