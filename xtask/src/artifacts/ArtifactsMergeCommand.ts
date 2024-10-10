import fs from 'node:fs/promises';
import path from 'node:path';
import { Command } from 'clipanion';
import glob from 'fast-glob';
import * as workspaceUtils from '../workspaceUtils';
import { ArtifactsConfig } from './config';

export class ArtifactsMergeCommand extends Command {
  static paths = [['artifacts', 'merge']];

  async execute() {
    const rootDir = workspaceUtils.findRootDir();
    const config = await ArtifactsConfig.load(path.join(rootDir, 'artifacts.json'));

    const allFilepaths = await Promise.all(
      config.files.map(({ source }) => {
        return glob(source, {
          cwd: rootDir,
          onlyFiles: true,
          followSymbolicLinks: false,
        });
      })
    );
    const filepaths = allFilepaths.flat();

    if (filepaths.length === 0) {
      this.context.stdout.write('No artifacts to merge');
      this.context.stdout.write('\n');
      return 0;
    }

    const totalNo = filepaths.length;
    this.context.stdout.write(`Found ${totalNo} file(s) to merge\n\n`);

    for (let i = 0; i < totalNo; i += 1) {
      const no = i + 1;
      const filepath = filepaths[i]!;
      const src = path.join(rootDir, filepath);
      const dest = path.join(config.absoluteDir, filepath);
      this.context.stdout.write(`[${no}/${totalNo}] Copy "${filepath}"\n`);
      await this.copyFile(src, dest);
    }

    return 0;
  }

  private async copyFile(src: string, dest: string) {
    const dir = path.dirname(dest);
    await fs.mkdir(dir, { recursive: true });
    await fs.copyFile(src, dest);
  }
}
