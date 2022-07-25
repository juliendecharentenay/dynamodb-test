# README #

This repository is used to investigate `aws-sdk-dynamodb` from the AWS Rust SDK - currently in developer 
preview.

# Lambda
This section relates to reproducing the observed issued in a minimal case running on AWS Lambda.

## Pre-requisites
* AWS cli
* Rust

## Setting-up dynamodb for testing
The following AWS cli commands are used to create a new dynamoDB table `DynamoDbTestTable`:

```
aws dynamodb create-table \
    --table-name DynamoDbTestTable \
    --billing-mode PAY_PER_REQUEST \
    --attribute-definitions AttributeName=ItemId,AttributeType=S \
    --key-schema AttributeName=ItemId,KeyType=HASH 
```

And add a couple of items:

```
aws dynamodb put-item --table-name DynamoDbTestTable --item file://./assets/item1.json
aws dynamodb put-item --table-name DynamoDbTestTable --item file://./assets/item2.json
aws dynamodb put-item --table-name DynamoDbTestTable --item file://./assets/item3.json
```

Using a local dynamodb installation:
```
aws dynamodb create-table \
    --endpoint-url http://localhost:8000 \
    --table-name DynamoDbTestTable \
    --billing-mode PAY_PER_REQUEST \
    --attribute-definitions AttributeName=ItemId,AttributeType=S \
    --key-schema AttributeName=ItemId,KeyType=HASH 

aws dynamodb put-item --endpoint-url http://localhost:8000 --table-name DynamoDbTestTable --item file://./assets/item1.json
aws dynamodb put-item --endpoint-url http://localhost:8000 --table-name DynamoDbTestTable --item file://./assets/item2.json
aws dynamodb put-item --endpoint-url http://localhost:8000 --table-name DynamoDbTestTable --item file://./assets/item3.json
```

## Compile the lambda function
It is assumed the compilation is undertaken using a linux OS. The following compilation flag allows
for static linking of the executable - to be self contained and suitable for the lambda provided
runtime environment:

```
cd lambda
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
```

## Create lambda function and upload executable
The AWS cli command create a lambda function:

```
# Create zip archive to upload
cp ./lambda/target/x86_64-unknown-linux-gnu/release/bootstrap .
zip lambda.zip ./bootstrap
rm -f bootstrap

# Create execution role
aws iam create-role \
  --role-name dynamodb-test-function-role \
  --assume-role-policy-document file://./assets/dynamodb-test-function-role.json
## export the role ARN using export ROLE_ARN=...

# Create policy
aws iam create-policy \
  --policy-name dynamodb-test-access-log-dynamodb-policy \
  --policy-document file://./assets/dynamodbtest-access-log-dynamodb-policy.json
## export the policy ARN using export POLICY_ARN=...

# Attach policy to the role
aws iam attach-role-policy \
  --role-name dynamodb-test-function-role \
  --policy-arn $POLICY_ARN
  
# Create lambda function
aws lambda create-function \
  --function-name dynamodb-test-function \
  --role $ROLE_ARN \
  --runtime provided \
  --handler bootstrap \
  --publish \
  --package-type Zip \
  --zip-file fileb://./lambda.zip

```

