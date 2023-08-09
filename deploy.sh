#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near dev-deploy ./target/wasm32-unknown-unknown/release/medibridge_contract_rust.wasm   new  '{}'  
# near deploy testing1111.testnet ./target/wasm32-unknown-unknown/release/medibridge_contract_rust.wasm   new  '{}'  