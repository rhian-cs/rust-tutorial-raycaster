# raycaster

This is my test repository where I followed this great blog post by @grantshandy: https://grantshandy.github.io/posts/raycasting/

## Dependencies

- Install `make`
- Install Rust Nightly
- Install [`binaryen`](https://github.com/WebAssembly/binaryen)
  - For Ubuntu you can install it with `sudo apt install binaryen`
- Install glibc version 2.31
  - For Ubuntu you can install it with `sudo apt install libc6`
- Install wasm4 version 2.3.1
  - With ASDF you can run:
  ```sh
  asdf plugin-add wasm4
  asdf install wasm4 2.3.1
  ```
- Enable the wasm architecture for Rust:
  - `rustup target add wasm32-unknown-unknown`

## Build and Run

Build with:

```
make
```

Build and run with:

```
make run
```

Build and check the size of the app with:

```
make size
```

## VS Code Tweaks

If you use VS Code and has the CodeLLDB extension installed, you can configure the workspace to run the app by pressing F5.

To do that just create a `.vscode/launch.json` directory and fill it with the following:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug",
      "program": "make",
      "args": ["run"],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

If your Rust Analyzer extension is annoying you about the panic handler, add the contents below to your local `config/settings.json` file ([source](https://github.com/rust-lang/rust-analyzer/issues/3801#issuecomment-1166269464)).

```json
{
  "rust-analyzer.checkOnSave.allTargets": false
}
```
