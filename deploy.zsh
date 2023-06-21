#!/bin/zsh

# build npm package
wasm-pack build --scope deanrumsby -- --features wasm &&

# publish to npm
npm publish pkg &&

# publish to crates.io
cargo publish &&

git tag -a $1 -m "Release $1" &&
git push --tags &&
echo "Published $1"
