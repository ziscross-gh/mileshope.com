#!/bin/bash
set -e

# Install Tailwind CSS for Cloudflare Pages (Linux x64)
if [ ! -f tailwindcss ]; then
  echo "Installing Tailwind CSS..."
  wget -q https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
  chmod +x tailwindcss-linux-x64
  mv tailwindcss-linux-x64 tailwindcss
fi

# Install Zola for Cloudflare Pages
ZOLA_VERSION="0.21.0"

echo "Installing Zola $ZOLA_VERSION..."
wget -q https://github.com/getzola/zola/releases/download/v${ZOLA_VERSION}/zola-v${ZOLA_VERSION}-x86_64-unknown-linux-gnu.tar.gz
tar xzf zola-v${ZOLA_VERSION}-x86_64-unknown-linux-gnu.tar.gz
chmod +x zola

# Build Tailwind CSS
echo "Building Tailwind CSS..."
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --minify

# Build Zola site
echo "Building site with Zola..."
./zola build

echo "Build complete!"
