# @wvb/electron-forge

## Usage with Yarn

These settings prevent Yarn's dependency layout and platform filtering from leaving required packages or native binaries out of the packaged app or DMG build.

- Use `nodeLinker: node-modules`; Forge does not support Yarn PnP.
- In workspaces, set the app's `installConfig.hoistingLimits` to `"workspaces"` so Forge can collect its dependencies.
- Set `packagerConfig.derefSymlinks: true` for workspace links and `asar.unpack: '**/*.node'` for native bindings.
- When building DMGs with multi-OS `supportedArchitectures`, upstream `fs-xattr` may be skipped.
  - Override it via `resolutions` with a pinned commit of [seokju-na/fs-xattr](https://github.com/seokju-na/fs-xattr/commit/750966126d2f8fd251be216d4eb4c8b4f85ce54c), as in this repository.

See more details in [playground/native/electron-forge](../../playground/native/electron-forge).
