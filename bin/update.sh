#!/bin/bash
set -e # Exit when command fails
ROOT_DIR=`pwd`

## Compile lambda
cd $ROOT_DIR/lambda
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

## Gather lambda in zip archive and update lambda function
cp ./target/x86_64-unknown-linux-gnu/release/bootstrap ./bootstrap
zip lambda.zip ./bootstrap
rm -f bootstrap
aws lambda update-function-code --function-name dynamodb-test-function --zip-file fileb://./lambda.zip
rm -f lambda.zip

