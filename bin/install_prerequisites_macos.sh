#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
brew install bash pkg-config squid
brew install https://raw.githubusercontent.com/Homebrew/homebrew-core/5f1b24e9882a1c1effa559a0000ff03ae155560b/Formula/curl.rb
PATH="/usr/local/opt/curl/bin/:$PATH"
export PATH
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

