# Donut

This project was inspired by the [Donut Math](https://www.a1k0n.net/2011/07/20/donut-math.html) article.

There are two folders in this project:

- **donut-cli**: Contains the code for a command-line interface (CLI) version of the donut project. To run this version, simply navigate to the `donut-cli` folder and run `cargo run`. The output will be displayed in the terminal.

- **donut-wasm**: Contains the code for a web assembly (WASM) version of the donut project. To build this version, navigate to the `donut-wasm` folder and run `wasm-pack build --target web`. This will create a WASM file that can be run in a web browser. To serve this file locally, you can use a simple HTTP server like `python3 -m http.server --bind 127.0.0.1`.


