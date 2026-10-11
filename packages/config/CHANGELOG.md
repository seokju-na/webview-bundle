# Changelog

## config v0.3.0

This release includes packages: [`@wvb/config`](https://www.npmjs.com/package/@wvb/config/v/0.3.0)

- fix(config,cli): remote `endpoint` -> `baseUrl` (#259) (1c31096)
- feat(config,cli): support setting remote config with funciton (#255) (5d763b9)
- feat(remote): update remote new model (#252) (6ad819d)
- feat(all): renew all update model (#236) (2cdde63)
- feat(cli): `extract` -> `unpack` rename command, add `pathResolver` option and tests (#235) (4e9fda6)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- fix: bundle integrity/signature verification & deterministic serialization (#196) (02018aa)
- refactor(cli,config): collapse pack outDir + outFileName into single outFile (#166) (7d67d53)
- feat(cli,config): add local builtin installation option (#160) (a2ef9f3)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- feat: redesgin cli commands, add local remote provider, oxc -> biome (#132) (af26b39)
- chore: fix xtask artifacts missing platforms (#112) (5d60ad6)
- update dependencies: wvb-node@0.3.0, @wvb/node@0.3.0, @wvb/node-android-arm-eabi@0.3.0, @wvb/node-android-arm64@0.3.0, @wvb/node-darwin-arm64@0.3.0, @wvb/node-darwin-x64@0.3.0, @wvb/node-linux-arm-gnueabihf@0.3.0, @wvb/node-linux-arm64-gnu@0.3.0, @wvb/node-linux-arm64-musl@0.3.0, @wvb/node-linux-x64-gnu@0.3.0, @wvb/node-linux-x64-musl@0.3.0, @wvb/node-win32-arm64-msvc@0.3.0, @wvb/node-win32-ia32-msvc@0.3.0, @wvb/node-win32-x64-msvc@0.3.0
