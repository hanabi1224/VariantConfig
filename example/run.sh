#!/bin/sh
pushd pages
yarn
yarn build
popd
cargo run
