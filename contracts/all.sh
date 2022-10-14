#! /bin/bash
set -e

sh optimize.sh && cd ../deploy/ && yarn deploy:all && cd ../app/ && yarn test
