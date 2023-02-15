#!/bin/sh

# install rust toolchain if necessary
if ! type cargo ; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

# install ginst
cargo install ginst
