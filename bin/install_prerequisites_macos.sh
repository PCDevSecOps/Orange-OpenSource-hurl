#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
curl -o curl.rb https://raw.githubusercontent.com/Homebrew/homebrew-core/884ae84ecf865873fd62c946d7f0a5b91a32c1bf/Formula/c/curl.rb
brew uninstall --force --ignore-dependencies curl
brew install -s curl.rb
PATH="/usr/local/opt/curl/bin/:$PATH"
export PATH
brew install bash pkg-config squid
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

