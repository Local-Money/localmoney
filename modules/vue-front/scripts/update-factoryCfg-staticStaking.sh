set -e

FACTORY=$(cat ../testsuite/cache/factory_bombay.json | jq '.factoryCfg.staking_addr = "terra1kjdz87qagw32cgmjduvv5q54728dm8ql27av6q" | .factoryCfg.xlocal_addr = "terra13j525m8yx9ts9gt7h98wm0c57d2hcu4apqafpq"') 

echo "Updated FactoryCfg to:"

echo "Hardcoded staking / xlocal !!"

echo "$FACTORY" | jq

echo "export const FACTORY_CFG = $FACTORY;" > src/constants/factoryCfg.js
