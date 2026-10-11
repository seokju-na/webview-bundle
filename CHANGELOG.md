# Changelog

## tauri v0.3.0

This release includes packages: [`wvb-tauri`](https://crates.io/crates/wvb-tauri/0.3.0)

- chore: set msrv (#269) (d7a77cc)
- feat(remote): update remote new model (#252) (6ad819d)
- refactor: update comments for reference documents (#239) (df5bd70)
- feat(all): renew all update model (#236) (2cdde63)
- feat: verify data checksums on read and bundles on load (#215) (76508f8)
- feat: add protocol resolver options (#214) (5d93c7f)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- feat(tauri): support iOS/Android mobile builds (#174) (f941058)
- fix: bundle integrity/signature verification & deterministic serialization (#196) (02018aa)
- feat(bridge,electron,tauri): remove legacy invokes to unify in `@wvb/bridge` and add `BridgeError` (#178) (f3735bb)
- fix(tauri): improve xvfb dri3 error to fix Intermittently e2e test fails (#177) (003123e)
- test(electron,tauri): update e2e tests with playground webview (#167) (b573da4)
- fix(tauri): fix build failed which cause from `alloc-no-stdlib` (#165) (896b0cd)
- fix(core): set total bytes as optional (#163) (7e38986)
- fix(tauri): fix resolve remote dir (#158) (4513cab)
- fix(tauri): fix cargo publish (#157) (6e31060)
- feat: update node.js, ffi, tauri api (#155) (a1bf387)
- feat: safe bundle update with separating download/install step and remove unused remote bundles after install (#152) (39020e6)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- feat: redesgin cli commands, add local remote provider, oxc -> biome (#132) (af26b39)
- feat: add core testing, tauri commands (#129) (c49207f)
- feat(core): add mock system for testing (#118) (7a79c1a)
- chore: fix xtask artifacts missing platforms (#112) (5d60ad6)
- update dependencies: wvb@0.3.0

## cloudflare v0.3.0

This release includes packages: [`@wvb/remote-cloudflare`](https://www.npmjs.com/package/@wvb/remote-cloudflare/v/0.3.0)

- chore(remote-aws,remote-cloudflare): set test timeout 10s (#256) (c838dd3)
- feat(remote): update remote new model (#252) (6ad819d)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- fix(cli,remote): fix confusing deploy command arguments, `--force` option for deploy was not working (#183) (9992fa1)
- fix(remote-cloudflare): set default region as `auto` (#182) (a5b67a4)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- feat: redesgin cli commands, add local remote provider, oxc -> biome (#132) (af26b39)
- chore: fix xtask artifacts missing platforms (#112) (5d60ad6)
- chore: fix xtask release scripts (#110) (745aca1)
- chore: migrate biome to oxc (oxlint/oxfmt) (#109) (59bdbeb)
- feat: renewal remote features (#96) (ea88b6b)
- feat(all): update remote providers (#87) (5fcdf96)
- feat(all): redesign manifest architecture, add config (#83) (10aa8d8)
- feat(all): separate signing/verifying signature and making/checking integrity features (#76) (12f4a76)
- refactor(remote): change remote spec (#66) (2acd598)
- feat(remote): add aws, cloudflare remote provider (#63) (aa7da40)
- update dependencies: @wvb/config@0.3.0, @wvb/remote-aws@0.3.0, @wvb/remote-base@0.3.0

## aws v0.3.0

This release includes packages: [`@wvb/remote-aws`](https://www.npmjs.com/package/@wvb/remote-aws/v/0.3.0)

- chore(remote-aws,remote-cloudflare): set test timeout 10s (#256) (c838dd3)
- feat(remote): update remote new model (#252) (6ad819d)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- fix(cli,remote): fix confusing deploy command arguments, `--force` option for deploy was not working (#183) (9992fa1)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- feat: redesgin cli commands, add local remote provider, oxc -> biome (#132) (af26b39)
- chore: fix xtask artifacts missing platforms (#112) (5d60ad6)
- chore: fix xtask release scripts (#110) (745aca1)
- chore: migrate biome to oxc (oxlint/oxfmt) (#109) (59bdbeb)
- feat: renewal remote features (#96) (ea88b6b)
- feat(all): update remote providers (#87) (5fcdf96)
- feat(all): redesign manifest architecture, add config (#83) (10aa8d8)
- feat(all): separate signing/verifying signature and making/checking integrity features (#76) (12f4a76)
- chore(deps): bump hono from 4.10.2 to 4.10.3 (#73) (e988f3c)
- chore(deps): bump hono from 4.9.7 to 4.10.2 (#72) (0e299ca)
- refactor(remote-aws,remote-aws-pulumi): separate lambda edge handlers as exports paths (#69) (bb46b1e)
- refactor(remote): change remote spec (#66) (2acd598)
- feat(remote): add aws, cloudflare remote provider (#63) (aa7da40)
- update dependencies: @wvb/config@0.3.0, @wvb/remote-base@0.3.0

## ffi v0.3.0

This release includes packages: 

- chore: set msrv (#269) (d7a77cc)
- fix(core): returns `None` when update returns 204 no contents (#268) (cf8b48f)
- feat(ffi): rustls platform verifier for android (#263) (f62e1f4)
- feat(remote): update remote new model (#252) (6ad819d)
- chore(ffi): update testapp min target to 15.0 (#246) (d3f64c0)
- refactor: update comments for reference documents (#239) (df5bd70)
- feat(all): renew all update model (#236) (2cdde63)
- fix: remove `hickory_dns` field in `HttpOptions` (#224) (76b4cbf)
- feat(ffi): Unify into a single BundleSourceConfig (#220) (b62b91c)
- feat: verify data checksums on read and bundles on load (#215) (76508f8)
- feat: add protocol resolver options (#214) (5d93c7f)
- feat: unify error codes across bindings (#212) (a4754f8)
- refactor(ffi): improve android e2e tests (#211) (ef225a5)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- fix(ffi): align e2e testTimeout with in-test waits (#208) (e7c4d74)
- feat(bridge,electron,tauri): remove legacy invokes to unify in `@wvb/bridge` and add `BridgeError` (#178) (f3735bb)
- feat: update node.js, ffi, tauri api (#155) (a1bf387)
- feat: safe bundle update with separating download/install step and remove unused remote bundles after install (#152) (39020e6)
- fix(ffi): fix missing xcframework asset (#147) (9a8f99e)
- feat: update xtask release commands which supports ffi (#141) (e0bfd59)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- update dependencies: wvb@0.3.0

## electron-forge v0.3.0

This release includes packages: [`@wvb/electron-forge`](https://www.npmjs.com/package/@wvb/electron-forge/v/0.3.0)

- fix(config,cli): remote `endpoint` -> `baseUrl` (#259) (1c31096)
- feat(remote): update remote new model (#252) (6ad819d)
- fix(electron-builder, electron-forge): defer builtin-empty check to install result (#226) (f83dced)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- feat(cli): add android/ios builtin presets (#190) (3c76b78)
- feat(electron-forge): add @wvb/electron-forge (#189) (d4221aa)
- update dependencies: @wvb/cli@0.3.0, @wvb/config@0.3.0

## electron-builder v0.3.0

This release includes packages: [`@wvb/electron-builder`](https://www.npmjs.com/package/@wvb/electron-builder/v/0.3.0)

- fix(config,cli): remote `endpoint` -> `baseUrl` (#259) (1c31096)
- feat(remote): update remote new model (#252) (6ad819d)
- fix(electron-builder, electron-forge): defer builtin-empty check to install result (#226) (f83dced)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- feat(cli): add android/ios builtin presets (#190) (3c76b78)
- feat(electron-builder): add @wvb/electron-builder (#188) (a604da9)
- update dependencies: @wvb/cli@0.3.0, @wvb/config@0.3.0

## electron v0.3.0

This release includes packages: [`@wvb/electron`](https://www.npmjs.com/package/@wvb/electron/v/0.3.0)

- chore(deps): bump the npm_and_yarn group across 1 directory with 8 updates (#240) (8571836)
- refactor: update comments for reference documents (#239) (df5bd70)
- feat(all): renew all update model (#236) (2cdde63)
- feat(electron): bundle desktop native binaries (#225) (db5a82c)
- feat: add playground stuffs (#217) (9a3ce92)
- feat: verify data checksums on read and bundles on load (#215) (76508f8)
- feat: add protocol resolver options (#214) (5d93c7f)
- feat: unify error codes across bindings (#212) (a4754f8)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- feat(bridge,electron,tauri): remove legacy invokes to unify in `@wvb/bridge` and add `BridgeError` (#178) (f3735bb)
- test(electron,tauri): update e2e tests with playground webview (#167) (b573da4)
- feat: update node.js, ffi, tauri api (#155) (a1bf387)
- chore(deps): bump the npm_and_yarn group across 4 directories with 9 updates (#140) (ce26eb2)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- feat: redesgin cli commands, add local remote provider, oxc -> biome (#132) (af26b39)
- feat: add core testing, tauri commands (#129) (c49207f)
- chore: fix xtask artifacts missing platforms (#112) (5d60ad6)
- update dependencies: wvb-node@0.3.0, @wvb/node@0.3.0, @wvb/node-android-arm-eabi@0.3.0, @wvb/node-android-arm64@0.3.0, @wvb/node-darwin-arm64@0.3.0, @wvb/node-darwin-x64@0.3.0, @wvb/node-linux-arm-gnueabihf@0.3.0, @wvb/node-linux-arm64-gnu@0.3.0, @wvb/node-linux-arm64-musl@0.3.0, @wvb/node-linux-x64-gnu@0.3.0, @wvb/node-linux-x64-musl@0.3.0, @wvb/node-win32-arm64-msvc@0.3.0, @wvb/node-win32-ia32-msvc@0.3.0, @wvb/node-win32-x64-msvc@0.3.0

## deno v0.3.0

This release includes packages: [`@wvb/deno`](https://jsr.io/@wvb/deno@0.3.0)

- feat(deno): extract builtin bundles cache at bootstrap (#266) (87a4604)
- feat(remote): update remote new model (#252) (6ad819d)
- feat(deno): merge single deno package `@wvb/deno-desktop` -> `@wvb/deno/desktop` (#250) (69809f5)
- refactor: update comments for reference documents (#239) (df5bd70)
- feat(all): renew all update model (#236) (2cdde63)
- feat(deno,deno-desktop): renaming interfaces, auto typegen with specta (#227) (d0ed365)
- fix: remove `hickory_dns` field in `HttpOptions` (#224) (76b4cbf)
- feat: verify data checksums on read and bundles on load (#215) (76508f8)
- feat: add protocol resolver options (#214) (5d93c7f)
- feat: unify error codes across bindings (#212) (a4754f8)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- chore: update justfile (#205) (262b4d9)
- feat(deno): add @wvb/deno and @wvb/deno-desktop (#187) (3c38d6c)
- update dependencies: wvb@0.3.0

## cli v0.3.0

This release includes packages: [`@wvb/cli`](https://www.npmjs.com/package/@wvb/cli/v/0.3.0)

- fix(config,cli): remote `endpoint` -> `baseUrl` (#259) (1c31096)
- fix(cli): builtin directory structure wrong (#257) (8e703b0)
- feat(config,cli): support setting remote config with funciton (#255) (5d763b9)
- fix(cli): inspect fail when logging with objects (#254) (82e08f6)
- feat(remote): update remote new model (#252) (6ad819d)
- feat(cli): update supported node version (#249) (38bee42)
- chore(deps): bump the npm_and_yarn group across 3 directories with 3 updates (#234) (e5c4611)
- feat(all): renew all update model (#236) (2cdde63)
- feat(cli): `extract` -> `unpack` rename command, add `pathResolver` option and tests (#235) (4e9fda6)
- refactor(cli): unify tsdown config (#219) (8da93ee)
- feat(cli): accept more color options (#218) (68c78ef)
- feat: verify data checksums on read and bundles on load (#215) (76508f8)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- feat(cli): add android/ios builtin presets (#190) (3c76b78)
- fix(cli): normalize config file default export to a plain object (#186) (562fb27)
- fix(cli): change `--version,-V` arguments in upload command (#185) (58bc641)
- fix(cli,remote): fix confusing deploy command arguments, `--force` option for deploy was not working (#183) (9992fa1)
- fix(cli): set `--deploy` option default value as false in `upload` command (#181) (439ff59)
- chore(deps): bump the npm_and_yarn group across 1 directory with 3 updates (#169) (dbc4e0e)
- refactor(cli,config): collapse pack outDir + outFileName into single outFile (#166) (7d67d53)
- fix(core): set total bytes as optional (#163) (7e38986)
- feat(cli,config): add local builtin installation option (#160) (a2ef9f3)
- chore(deps): bump hono from 4.12.18 to 4.12.21 in the npm_and_yarn group across 1 directory (#156) (f70129a)
- chore(deps): bump the npm_and_yarn group across 4 directories with 9 updates (#140) (ce26eb2)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- feat: redesgin cli commands, add local remote provider, oxc -> biome (#132) (af26b39)
- feat(cli): expose `/api` import path for `@wvb/cli` npm package (#122) (fea4e32)
- chore(deps): bump the npm_and_yarn group across 1 directory with 2 updates (#119) (722dcd0)
- chore: fix xtask artifacts missing platforms (#112) (5d60ad6)
- update dependencies: @wvb/config@0.3.0, wvb-node@0.3.0, @wvb/node@0.3.0, @wvb/node-android-arm-eabi@0.3.0, @wvb/node-android-arm64@0.3.0, @wvb/node-darwin-arm64@0.3.0, @wvb/node-darwin-x64@0.3.0, @wvb/node-linux-arm-gnueabihf@0.3.0, @wvb/node-linux-arm64-gnu@0.3.0, @wvb/node-linux-arm64-musl@0.3.0, @wvb/node-linux-x64-gnu@0.3.0, @wvb/node-linux-x64-musl@0.3.0, @wvb/node-win32-arm64-msvc@0.3.0, @wvb/node-win32-ia32-msvc@0.3.0, @wvb/node-win32-x64-msvc@0.3.0, @wvb/remote-local@0.3.0

## local v0.3.0

This release includes packages: [`@wvb/remote-local`](https://www.npmjs.com/package/@wvb/remote-local/v/0.3.0)

- feat(remote): update remote new model (#252) (6ad819d)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- fix(remote-local): add missing `content-length` header (#162) (0c184ac)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- feat: redesgin cli commands, add local remote provider, oxc -> biome (#132) (af26b39)
- update dependencies: @wvb/config@0.3.0, @wvb/remote-base@0.3.0

## base v0.3.0

This release includes packages: [`@wvb/remote-base`](https://www.npmjs.com/package/@wvb/remote-base/v/0.3.0)

- feat(remote): update remote new model (#252) (6ad819d)
- update dependencies: @wvb/config@0.3.0

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

## core v0.3.0

This release includes packages: [`wvb`](https://crates.io/crates/wvb/0.3.0)

- chore: set msrv (#269) (d7a77cc)
- fix(core): returns `None` when update returns 204 no contents (#268) (cf8b48f)
- fix(core): source dir documentation wrong (#258) (d898301)
- feat(remote): update remote new model (#252) (6ad819d)
- fix(core): remove `local-offset` feature in `time` crate (#247) (cacd1af)
- refactor: update comments for reference documents (#239) (df5bd70)
- feat(all): renew all update model (#236) (2cdde63)
- fix(core): handle Windows delete-pending bundle reads (#229) (ce62907)
- fix: remove `hickory_dns` field in `HttpOptions` (#224) (76b4cbf)
- feat: verify data checksums on read and bundles on load (#215) (76508f8)
- feat: add protocol resolver options (#214) (5d93c7f)
- feat: unify error codes across bindings (#212) (a4754f8)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- fix(core): fix windows fs retry (#201) (1d1c856)
- fix(core): retry manifest read on Windows transient errors during concurrent rename (#197) (5ae0375)
- fix: bundle integrity/signature verification & deterministic serialization (#196) (02018aa)
- refactor(core): update deprecated method for time formatting (#184) (8b4d8aa)
- fix(core): set total bytes as optional (#163) (7e38986)
- feat: update node.js, ffi, tauri api (#155) (a1bf387)
- feat: safe bundle update with separating download/install step and remove unused remote bundles after install (#152) (39020e6)
- feat(ffi): add ffi package for Android/iOS bindings (#139) (c41b723)
- fix(core): pass request headers to local proxy (#130) (b8ef163)
- feat: add core testing, tauri commands (#129) (c49207f)
- feat(core): add tracing support for debugging and request handling (#123) (5a4ed09)
- feat(core): add mock system for testing (#118) (7a79c1a)

## bridge v0.3.0

This release includes packages: [`@wvb/bridge`](https://www.npmjs.com/package/@wvb/bridge/v/0.3.0)

- feat(remote): update remote new model (#252) (6ad819d)
- feat(all): renew all update model (#236) (2cdde63)
- feat(electron): bundle desktop native binaries (#225) (db5a82c)
- refactor(xtask): move package config to root xtask.config.ts (#207) (1b3f0ab)
- feat(deno): add @wvb/deno and @wvb/deno-desktop (#187) (3c38d6c)
- feat(bridge,electron,tauri): remove legacy invokes to unify in `@wvb/bridge` and add `BridgeError` (#178) (f3735bb)
- feat(bridge): impl universal bridge (#171) (e4cfd0d)
