# Changelog

## node v0.3.1

This release includes packages: [`@wvb/node`](https://www.npmjs.com/package/@wvb/node/v/0.3.1), [`@wvb/node-android-arm-eabi`](https://www.npmjs.com/package/@wvb/node-android-arm-eabi/v/0.3.1), [`@wvb/node-android-arm64`](https://www.npmjs.com/package/@wvb/node-android-arm64/v/0.3.1), [`@wvb/node-darwin-arm64`](https://www.npmjs.com/package/@wvb/node-darwin-arm64/v/0.3.1), [`@wvb/node-darwin-x64`](https://www.npmjs.com/package/@wvb/node-darwin-x64/v/0.3.1), [`@wvb/node-linux-arm-gnueabihf`](https://www.npmjs.com/package/@wvb/node-linux-arm-gnueabihf/v/0.3.1), [`@wvb/node-linux-arm64-gnu`](https://www.npmjs.com/package/@wvb/node-linux-arm64-gnu/v/0.3.1), [`@wvb/node-linux-arm64-musl`](https://www.npmjs.com/package/@wvb/node-linux-arm64-musl/v/0.3.1), [`@wvb/node-linux-x64-gnu`](https://www.npmjs.com/package/@wvb/node-linux-x64-gnu/v/0.3.1), [`@wvb/node-linux-x64-musl`](https://www.npmjs.com/package/@wvb/node-linux-x64-musl/v/0.3.1), [`@wvb/node-win32-arm64-msvc`](https://www.npmjs.com/package/@wvb/node-win32-arm64-msvc/v/0.3.1), [`@wvb/node-win32-ia32-msvc`](https://www.npmjs.com/package/@wvb/node-win32-ia32-msvc/v/0.3.1), [`@wvb/node-win32-x64-msvc`](https://www.npmjs.com/package/@wvb/node-win32-x64-msvc/v/0.3.1)

- fix(node): match expect package version from `package.json` file (#290) (338139e)

## node v0.3.0

This release includes packages: [`@wvb/node`](https://www.npmjs.com/package/@wvb/node/v/0.3.0), [`@wvb/node-android-arm-eabi`](https://www.npmjs.com/package/@wvb/node-android-arm-eabi/v/0.3.0), [`@wvb/node-android-arm64`](https://www.npmjs.com/package/@wvb/node-android-arm64/v/0.3.0), [`@wvb/node-darwin-arm64`](https://www.npmjs.com/package/@wvb/node-darwin-arm64/v/0.3.0), [`@wvb/node-darwin-x64`](https://www.npmjs.com/package/@wvb/node-darwin-x64/v/0.3.0), [`@wvb/node-linux-arm-gnueabihf`](https://www.npmjs.com/package/@wvb/node-linux-arm-gnueabihf/v/0.3.0), [`@wvb/node-linux-arm64-gnu`](https://www.npmjs.com/package/@wvb/node-linux-arm64-gnu/v/0.3.0), [`@wvb/node-linux-arm64-musl`](https://www.npmjs.com/package/@wvb/node-linux-arm64-musl/v/0.3.0), [`@wvb/node-linux-x64-gnu`](https://www.npmjs.com/package/@wvb/node-linux-x64-gnu/v/0.3.0), [`@wvb/node-linux-x64-musl`](https://www.npmjs.com/package/@wvb/node-linux-x64-musl/v/0.3.0), [`@wvb/node-win32-arm64-msvc`](https://www.npmjs.com/package/@wvb/node-win32-arm64-msvc/v/0.3.0), [`@wvb/node-win32-ia32-msvc`](https://www.npmjs.com/package/@wvb/node-win32-ia32-msvc/v/0.3.0), [`@wvb/node-win32-x64-msvc`](https://www.npmjs.com/package/@wvb/node-win32-x64-msvc/v/0.3.0)

- chore: set msrv (#269) (d7a77cc)
- fix(node): update dts file (#262) (e030bf6)
- feat(remote): update remote new model (#252) (6ad819d)
- fix(node): update node bidning dts (#251) (44f736e)
- refactor: update comments for reference documents (#239) (df5bd70)
- chore(deps): bump the npm_and_yarn group across 3 directories with 3 updates (#234) (e5c4611)
- feat(all): renew all update model (#236) (2cdde63)
- feat(electron): bundle desktop native binaries (#225) (db5a82c)
- fix: remove `hickory_dns` field in `HttpOptions` (#224) (76b4cbf)
- feat: verify data checksums on read and bundles on load (#215) (76508f8)
- feat: add protocol resolver options (#214) (5d93c7f)
- feat: unify error codes across bindings (#212) (a4754f8)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- fix: bundle integrity/signature verification & deterministic serialization (#196) (02018aa)
- chore(deps): bump the npm_and_yarn group across 1 directory with 3 updates (#169) (dbc4e0e)
- fix(core): set total bytes as optional (#163) (7e38986)
- feat: update node.js, ffi, tauri api (#155) (a1bf387)
- chore(deps): bump hono from 4.12.18 to 4.12.21 in the npm_and_yarn group across 1 directory (#156) (f70129a)
- feat: safe bundle update with separating download/install step and remove unused remote bundles after install (#152) (39020e6)
- chore(deps): bump the npm_and_yarn group across 4 directories with 9 updates (#140) (ce26eb2)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- feat: redesgin cli commands, add local remote provider, oxc -> biome (#132) (af26b39)
- chore(deps): bump the npm_and_yarn group across 1 directory with 2 updates (#119) (722dcd0)
- feat(core): add mock system for testing (#118) (7a79c1a)
- fix(xtask): fix commit changes (#114) (4c80929)
- update dependencies: wvb@0.3.0
