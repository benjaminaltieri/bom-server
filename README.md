# BOM Server
A Simple Rest API to manage a set of BOMs built in Rust using Rocket

# How to Build

## Install Rust
If you don't already have rust installed you can do so easily by following the instructions on the [rustup website](https://rustup.rs/) which will instruct you to run command below:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once you have `rustup` installed, switch to the nightly version either as default globally or within this source directory:

```
# use nightly rust system-wide
rustup default nightly
# or just within this directory
rustup override set nightly
```

## Using Cargo to Build and Run Tests
Once you have the tooling installed building and running the server and client is pretty straightforward:

```
cargo build
cargo test
cargo run --bin bom-server
cargo run --bin bom-client -- --help
```

# Server
After building, you can run the server using the following command:

```
cargo run --bin bom-server
```

This is automatically begin serving at port 8000 on the localhost address. You can use a web browser to check liveness by visiting `http://localhost:8000`.

# Client
Any http client can be used to send and receive json blobs to the server, but a client exists to streamline this for testing purposes. Run using the following the command, which will show the help text:

```
cargo run --bin bom-client -- --help
``` 

The each subcommand is fully documented which can be viewed by inserting the subcommand prior to `--help`:
```
cargo run --bin bom-client -- <subcommand> --help
``` 

# Testing
The is a convenience script located under the `test` folder which you can run to populate the server with an example configuration of parts:

```
./test/populate_server.sh
```

The server must be running prior to executing the above script, and must be restarted prior to running the script again.

Once loaded, the provided configuration of parts allows any number of operations to be performed including various filtered list requested and removal or updating of the BOM relationships on the server.

