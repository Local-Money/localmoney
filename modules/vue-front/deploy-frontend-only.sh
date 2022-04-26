set -e

cd ../contracts

./optimize.sh

cd ../testsuite

npm run upload:bombay:arbitrationInterface

cd ../vue-front

FACTORY=$(cat ../testsuite/cache/factory_bombay.json | jq '.factoryAddr') 

echo "export const FACTORY_CONTRACT = $FACTORY;" > src/constants.js

yarn build

git add -A

git commit -m 'deploy-vue-front'

git push -f git@github.com:Local-Terra/localterra.git main:deploy

ssh root@testnet.localterra.money "cd /var/www/html/localterra; git checkout deploy; git pull"
