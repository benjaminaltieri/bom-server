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

