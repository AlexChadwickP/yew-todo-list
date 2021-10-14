# Yew Todo List
This is a front-end todo list application written in Rust using the yew front-end framework for Rust and web_sys.

## How to run
If you've never used yew, wasm or trunk run the following:
First install the cargo dependencies
```
cargo install trunk wasm-bindgen-cli
```
Then install the wasm target with Rustup
```
rustup target add wasm32-unknown-unknown
```
And then use this command to start a development server
```
trunk serve
```

