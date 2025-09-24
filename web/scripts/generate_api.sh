#!/bin/sh

rm -rf src/api/generated

(cd ../api && TS_RS_EXPORT_DIR=../web/src/api/generated cargo test export_bindings)

pnpm run check --write ./src/api/generated
