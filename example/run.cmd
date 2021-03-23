pushd %~dp0pages
yarn && yarn build && cd .. && cargo run
popd
