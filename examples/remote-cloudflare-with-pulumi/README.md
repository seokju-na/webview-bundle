# Cloudflare Pulumi Template

A minimal Webview Bundle Remote Provider for Cloudflare

## Prerequisites

- Pulumi CLI (>= v3): https://www.pulumi.com/docs/get-started/install/
- Node.js (>= 14): https://nodejs.org/
- Cloudflare credentials configured

## Credentials

Set the following values:

```shell
$ pulumi config set accountId <CLOUDFLARE_ACCOUNT_ID>
$ pulumi config set cloudflare:apiToken <CLOUDFLARE_API_TOKEN> --secret
```

See detail configuration via https://www.pulumi.com/registry/packages/cloudflare/installation-configuration/
