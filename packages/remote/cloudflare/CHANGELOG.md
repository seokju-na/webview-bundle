# Changelog

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
