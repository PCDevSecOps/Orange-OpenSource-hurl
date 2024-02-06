#!/bin/bash
set -Eeuo pipefail

echo "----- install prerequisite packages -----"
brew install bash pkg-config squid curl@8.5.0
PATH="/usr/local/opt/curl/bin/:$PATH"
export PATH
sudo squid -k shutdown || true
sudo rm -v /dev/shm/squid*.shm >/dev/null 2>&1 || true

