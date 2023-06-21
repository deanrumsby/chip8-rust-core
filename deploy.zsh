#!/bin/zsh

# build npm package
wasm-pack build --scope deanrumsby -- --features wasm &&

# publish to npm
cd pkg &&
npm publish &&

# publish to crates.io
cd ../ &&
cargo publish &&

git tag -a $1 -m "Release $1" &&
git push --tags &&
echo "Published $1"
