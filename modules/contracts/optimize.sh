#! /bin/bash
set -e

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6

DATE=$(date -u +"%Y-%m-%d-%H_%M_%S_UTC") # Date in UTC
echo "$DATE"

git commit -a -m "build $DATE" || true
git tag "build_$DATE"
