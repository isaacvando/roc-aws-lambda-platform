#!/bin/bash
set -eo pipefail

if [ -z "$1" ]; then
    echo "Please provide a function name"
    echo "Usage: $0 my_lambda"
    exit 1
fi

name="$1"

set -u

# Check if the lambda already exists
if aws lambda get-function --function-name "$name" > /dev/null
then
    aws lambda update-function-code --function-name "$name" --zip-file fileb://bootstrap.zip > /dev/null
else
    aws lambda create-function --function-name "$name" \
      --handler bootstrap \
      --zip-file fileb://bootstrap.zip \
      --runtime provided.al2023 \
      --role arn:aws:iam::649730264873:role/LambdaAdmin \
      --environment 'Variables={RUST_BACKTRACE=1}' \
      --tracing-config Mode=Active \
      > /dev/null
fi

echo "Deployed $name to AWS"
