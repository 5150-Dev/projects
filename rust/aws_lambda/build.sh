#!/bin/sh

mode=""

if [ "$1" = "release" ]; then
  mode="--release"
fi

cd lambda_bootstrap
cargo build ${mode}
cd ../lambda_test
./build.sh $1

exit 0
