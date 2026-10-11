# AWS remote with Pulumi

Uses `WvbRemoteProvider` from `@wvb/remote-aws/pulumi` to create an S3 bucket,
CloudFront distribution, published Lambda@Edge handler, and IAM permissions.

---

Install the [Pulumi CLI](https://www.pulumi.com/docs/iac/cli/) and configure AWS credentials, for example through your
existing `AWS_PROFILE`. From the repository root:

```sh
cd playground/remote/aws-with-pulumi
pulumi stack init <STACK_NAME>
pulumi config set aws:region <REGION>
pulumi config set bucketName <BUCKET_NAME>
pulumi preview
pulumi up
```
