#!/bin/sh

mode=""
dir="debug"

if [ "$1" = "release" ]; then
  echo "Building for release"
  mode="--release"
  dir="release"
else
  echo "Building for debug"
fi

cargo build ${mode}
mv target/${dir}/lambda_test ./build/lambda
cp ./bootstrap ./build
cd build
zip -r bootstrap.zip ./