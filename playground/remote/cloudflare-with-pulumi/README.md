# Cloudflare remote with Pulumi

Uses `WvbRemoteProvider` from `@wvb/remote-cloudflare/pulumi` to provision:

---

Install the [Pulumi CLI](https://www.pulumi.com/docs/iac/cli/) and configure a state backend with `pulumi login`.
The Cloudflare account must have R2 and a `workers.dev` subdomain enabled.
Use a Cloudflare API token with permission to manage R2 buckets, Workers KV,
and Workers scripts in that account.

```sh
cd playground/remote/cloudflare-with-pulumi
pulumi stack init <STACK_NAME>
pulumi config set accountId <CLOUDFLARE_ACCOUNT_ID>
pulumi config set --secret cloudflare:apiToken
pulumi preview
pulumi up
```
