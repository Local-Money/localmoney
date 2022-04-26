set -e


yarn build

git add -A

git commit -m 'deploy-vue-front'

git push -f git@github.com:Local-Terra/localterra.git main:deploy

ssh root@testnet.localterra.money "cd /var/www/html/localterra; git checkout deploy; git pull"
