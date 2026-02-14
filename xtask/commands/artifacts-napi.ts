import { Command, Option } from 'clipanion';
import glob from 'fast-glob';
import fs from 'node:fs/promises';
import path from 'node:path';
import { ColorModeOption, colors, setColorMode } from '../console.ts';
import { ROOT_DIR } from '../consts.ts';

export class ArtifactsNapiCommand extends Command {
  static paths = [['artifacts', 'napi']];

  readonly relativePath = Option.String();
  readonly colorMode = ColorModeOption;

  async execute() {
    setColorMode(this.colorMode);
    try {
      const cwd = path.join(ROOT_DIR, this.relativePath);
      const files = await glob('./artifacts/*.node', {
        cwd,
        onlyFiles: true,
      });
      if (files.length === 0) {
        console.log(colors.warn('no files found. skip.'));
        return 0;
      }
      console.log(`found ${colors.info(files.length)} file(s) to make artifacts`);
      for (let i = 0; i < files.length; i += 1) {
        const progress = `[${i + 1}/${files.length}]`;
        const file = files[i]!;
        const src = path.join(cwd, file);
        const dest = this.resolveNodeFilepath(src);
        await fs.mkdir(path.dirname(dest), { recursive: true });
        await fs.copyFile(src, dest);
        console.log(`${colors.success(progress)} ${path.relative(cwd, dest)}: file copied`);
      }
      return 0;
    } catch (e) {
      console.error(e);
      return 1;
    }
  }

  private resolveNodeFilepath(filepath: string) {
    const dirname = path.dirname(filepath);
    const filename = path.basename(filepath);
    const [name, arch, ext] = filename.split('.');
    switch (`${arch}.${ext}`) {
      case 'darwin-x64.node':
        return path.join(dirname, `${name}-x86_64-apple-darwin`, filename);
      case 'darwin-arm64.node':
        return path.join(dirname, `${name}-aarch64-apple-darwin`, filename);
      case 'win32-x64-msvc.node':
        return path.join(dirname, `${name}-x86_64-pc-windows-msvc`, filename);
      case 'win32-arm64-msvc.node':
        return path.join(dirname, `${name}-aarch64-pc-windows-msvc`, filename);
      case 'linux-x64-musl.node':
        return path.join(dirname, `${name}-x86_64-unknown-linux-musl`, filename);
      case 'linux-x64-gnu.node':
        return path.join(dirname, `${name}-x86_64-unknown-linux-gnu`, filename);
      case 'linux-arm64-musl.node':
        return path.join(dirname, `${name}-aarch64-unknown-linux-musl`, filename);
      case 'linux-arm64-gnu.node':
        return path.join(dirname, `${name}-aarch64-unknown-linux-gnu`, filename);
      default:
        throw new Error(`unknown file: ${filepath}`);
    }
  }
}
