set -e
cd ../contracts
./optimize.sh
cd ../testsuite
npm run upload:bombay:arbitrationInterface
cd ../vue-front
yarn build

git add -A

git commit -m 'deploy-vue-front'

git push -f git@github.com:Local-Terra/localterra.git master:gh-pages

# pwd

# cd 