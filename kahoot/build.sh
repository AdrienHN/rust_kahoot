#! /bin/bash

wasm-pack build --target web
./node_modules/rollup/dist/bin/rollup ./main.js --format iife --file ./pkg/bundle.js
./node_modules/rollup/dist/bin/rollup ./login.js --format iife --file ./pkg/index-bundle.js