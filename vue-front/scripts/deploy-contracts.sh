set -e

cd ../contracts

./optimize.sh

cd ../testsuite

npm run bombay:deploy