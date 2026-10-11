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
