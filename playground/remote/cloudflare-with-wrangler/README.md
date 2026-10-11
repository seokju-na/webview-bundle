# Cloudflare remote with Wrangler

Uses `@wvb/remote-cloudflare/worker` with Wrangler to provision and bind:

---

From the repository root:

```sh
cd playground/remote/cloudflare-with-wrangler
yarn wrangler login
yarn types
yarn dev
```

`yarn dev` uses local R2/KV storage. To provision remote resources and deploy:

```sh
yarn preview
yarn deploy
```

Wrangler automatically creates missing R2/KV resources during deployment and
writes provisioned binding identifiers into the configuration.
