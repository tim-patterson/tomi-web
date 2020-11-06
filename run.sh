set -e;
wasm-pack build --target web --out-dir target/pkg
mkdir -p target/web
cp target/pkg/tomi_web.js target/pkg/tomi_web_bg.wasm target/web
cp -r static/* target/web
miniserve target/web/ --index index.html