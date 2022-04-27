set -e

FACTORY=$(cat ../testsuite/cache/factory_bombay.json | jq '.factoryAddr') 

echo "export const FACTORY_CONTRACT = $FACTORY;" > src/constants.js
