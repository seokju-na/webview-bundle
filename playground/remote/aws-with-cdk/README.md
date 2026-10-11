# AWS remote with CDK

Uses `WebviewBundleRemote` from `@wvb/remote-aws/cdk` to create a private S3 bucket,
CloudFront distribution with origin access control, and a Lambda@Edge handler.

---

First, write the context file (`cdk.context.json`):

```json
{
  "bucketName": "<YOUR_BUCKET_NAME>",
  "region": "<AWS_REGION> [Default: us-east-1]"
}
```

Configure AWS credentials (for example `AWS_PROFILE`) and then follow the below commands:

```sh
cd playground/remote/aws-with-cdk
yarn cdk bootstrap
yarn cdk synth --strict
yarn cdk diff
yarn cdk deploy --all
```
