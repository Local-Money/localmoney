#! /bin/bash
set -e

# Version that supports arm and x86_64
V=0.12.8

# Gets the machine architecture
M=$(uname -m)
S=${M#x86_64}
S=${S:+-$S}


docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer$S:$V
