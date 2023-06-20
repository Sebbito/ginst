#!/bin/env bash

if [[ ! -d target/built ]]; then
    mkdir -p target/built
fi

for target in `dir target/`
do
    if [[ $target != "CACHEDIR.TAG" ]] && [[ $target != "release" ]]; then
        mv "target/$target/release/ginst" "target/built/ginst-$target"
    fi
done
