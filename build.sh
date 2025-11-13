#!/bin/bash

# Install Zola for Cloudflare Pages
ZOLA_VERSION="0.21.0"

echo "Installing Zola $ZOLA_VERSION..."
wget -q https://github.com/getzola/zola/releases/download/v${ZOLA_VERSION}/zola-v${ZOLA_VERSION}-x86_64-unknown-linux-gnu.tar.gz
tar xzf zola-v${ZOLA_VERSION}-x86_64-unknown-linux-gnu.tar.gz
chmod +x zola

echo "Building site with Zola..."
./zola build

echo "Build complete!"
