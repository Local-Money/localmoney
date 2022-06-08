set -e

FACTORY=$(cat ../testsuite/cache/factory_bombay.json | jq '.factoryCfg.staking_addr = "terra1hepfmtzj3uulgap59ygn2pae86lxnq4knwcg3g" | .factoryCfg.xlocal_addr = "terra1q0ynhurhgpc2vcqhhv36g3qcqlyy8mvptg4xt4"') 

echo "Updated FactoryCfg to:"

echo "Hardcoded staking / xlocal !!"

echo "$FACTORY" | jq

echo "export const FACTORY_CFG = $FACTORY;" > src/constants/factoryCfg.js
