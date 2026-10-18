# create-wvb

Create a Webview Bundle project with Node.js 22.12 or later, or Deno 2.9.7 or later.

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
npm create wvb@latest my-project -- --template electron-forge

# yarn
yarn create wvb my-project --template electron-forge

# pnpm
pnpm create wvb my-project --template electron-forge
```

Currently supported template presets include:

- `electron`
- `electron-forge`
- `electron-builder`
- `tauri`
- `android`
- `ios`
- `deno-desktop`

## License

MIT License
