# create-wvb

Requires [Node.js](https://nodejs.org) version 22.12 or later.

Command usage:

```bash
# npm
npm create wvb@latest

# yarn
yarn create wvb

# pnpm
pnpm create wvb
```

Then follow the prompts.

---

Specify the project name and the template via additional command line options.
For example, to scaffold a Electron Forge project, run:

```bash
# npm 7+, extra double-dash is needed
npm create wvb@latest my-wvb -- --template electron-forge

# yarn
yarn create wvb my-wvb --template electron-forge

# pnpm
pnpm create wvb my-wvb --template electron-forge
```

Currently supported template presets include:

- `electron`
- `electron-forge`
- `electron-builder`
- `tauri`
- `android`
- `ios`
- `deno-desktop`

## Generated projects

README commands follow the package manager that invokes `create-wvb` (npm, Yarn, pnpm, or Bun). Direct Node invocation defaults to npm. `.yarnrc.yml` is generated only for Yarn 2 or later, based on the invoking package manager's version.

Android and iOS names are derived from the project directory using Mustache templates. For example, `my-demo` generates the app name `MyDemo` and identifier `com.example.appmydemo`. Names starting with a digit receive an `App` prefix. Names without ASCII letters or digits use an `App` prefix and a hash of the project name.

Deno projects use `deno.json` and `deno task`; they have no root `package.json`. `webview/package.json` remains as the local bundle's name and version metadata for `wvb builtin`.

The Electron Forge template uses the Forge Vite plugin for main and preload builds. `@wvb/electron` remains external and is included as a runtime dependency in packaged apps.

## License

MIT License
