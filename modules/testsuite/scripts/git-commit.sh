
#! /bin/bash

DATE=$(date -u +"%Y-%m-%d-%H_%M_%S_UTC") # Date in UTC

git commit -a -m "upload-inst $DATE" || true
git tag "upload-inst_$DATE"