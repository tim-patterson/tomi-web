#!/bin/sh
set -e

wasm-pack build --target web --out-dir target/pkg
mkdir -p target/web
cp target/pkg/tomi_web.js target/pkg/tomi_web_bg.wasm target/web
cp -r static/* target/web

REMOTE=$(git remote get-url origin)
(
  cd target/web
  git init
  git remote add origin $REMOTE
  git add .
  git commit -m "GH PAGES"
  git push --force origin master:gh-pages
)
echo "Deploy Done"