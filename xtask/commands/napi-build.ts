import { NapiCli } from '@napi-rs/cli';
import { Command } from 'clipanion';

export class NapiBuildCommand extends Command {
  static paths = [['napi-build']];

  async execute() {
    const napi = new NapiCli();
    await napi.build({
      platform: true,
      release: true,
      jsBinding: 'binding.js',
      dts: 'binding.d.ts',
      cwd: process.cwd(),
      manifestPath: './Cargo.toml',
      esm: true,
      constEnum: false,
      dtsCache: false,
    });
    await napi.build({
      platform: true,
      release: true,
      jsBinding: 'binding.cjs',
      dts: 'binding.d.cts',
      esm: false,
      constEnum: false,
      dtsCache: false,
    });
  }
}
