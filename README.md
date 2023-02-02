# rust-hello-wasm
Very basic rust wasm (WebAssembly) program

Refer: https://www.youtube.com/watch?v=nW71Mlbmxt8

## Information
- We are using the wasm-bindgen crate, which helps translate between the WebAssembly types and native JS types and the rust types.
- In the [lib] in toml file we are passing crate-type as "cdylib" which is the c dynamic library.

## To get the wasm compiled for JS
```bash
wasm-pack build --target web
```

- One amazing thing is it also generate the type declaration files as well, so that our wasm function will actually be correctly typed.
- Now the index.html cannot be loaded from the file system. So, we can use the lite-server for this.

## How to use this?
Step 1: Clone the repo