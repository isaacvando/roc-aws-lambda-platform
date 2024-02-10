set -euo pipefail

# Check if the lambda already exists
if aws lambda get-function --function-name $1 > /dev/null
then
    aws lambda update-function-code --function-name $1 --zip-file fileb://bootstrap.zip > /dev/null
else
    aws lambda create-function --function-name $1 \
      --handler bootstrap \
      --zip-file fileb://bootstrap.zip \
      --runtime provided.al2023 \
      --role arn:aws:iam::649730264873:role/LambdaAdmin \
      --environment Variables={RUST_BACKTRACE=1} \
      --tracing-config Mode=Active
      > /dev/null
fi

echo "Deployed $1 to AWS"
