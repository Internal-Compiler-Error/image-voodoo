all:
    wasm-pack build
    cd next/ && npm install
