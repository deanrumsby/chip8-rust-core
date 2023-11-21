#!/bin/sh

# build npm package
wasm-pack build --scope deanrumsby -- --features wasm &&

# dry run for crates.io
cargo publish --features std --dry-run &&

# dry run for npm
cd pkg &&
npm publish --dry-run &&

# publish to npm
npm publish &&

# publish to crates.io
cd ../ &&
cargo publish --features std &&

git tag -a $1 -m "Release $1" &&
git push --tags &&
echo "Published $1"
