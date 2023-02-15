#!/bin/bash

script_dir=$(dirname $(realpath $0))
project_root=$(dirname $script_dir)
path="$project_root/Cargo.toml"

if [ $# -eq 1 ]; then
    if [[ $1 =~ ^[0-9]\.[0-9]\.[0-9]$ ]]; then
        ver=$1
    else
        echo "Invalid format. Please use x.x.x as the version number format)."
        exit 1
    fi
else
    echo "Please enter a version number (x.x.x - Format)."
    exit 1
fi

if [ -f $path ]; then
    sed -i "s/^version = \"[0-9\.]*\"$/version = \"$ver\"/" "$path"
else
    echo "Path $path doesn't exist"
    exit 1
fi

$project_root/test.sh

exit 0
