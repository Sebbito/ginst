#!/bin/bash

script_dir=$(dirname $(realpath $0))
project_root=$(dirname $script_dir)

path="$project_root/ginst/Cargo.toml"

if [ $# -eq 1 ]; then
    if [[ $1 =~ ^v[0-9]\.[0-9]\.[0-9]$ ]]; then
        ver=$1
    else
        echo "Invalid format. Please use vx.x.x as the version number format)."
        exit 1
    fi
else
    echo "Please enter a version number (vx.x.x - Format)."
    exit 1
fi

if [ -f $path ]; then
    sed -i "s/^version = \"[v0-9\.]*\"$/version = \"$ver\"/" "$path"
else
    echo "Path $path doesn't exist"
    exit 1
fi

exit 0
