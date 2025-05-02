import fs from 'node:fs/promises';
import path from 'node:path';
import { Command } from 'clipanion';
import glob from 'fast-glob';
import { minimatch } from 'minimatch';
import * as workspaceUtils from '../workspaceUtils';
import { ArtifactsConfig } from './config';

export class ArtifactsSpreadCommand extends Command {
  static paths = [['artifacts', 'spread']];

  async execute() {
    const rootDir = workspaceUtils.findRootDir();
    const config = await ArtifactsConfig.load(path.join(rootDir, 'artifacts.json'));

    const filepaths = await glob('**/*', {
      cwd: config.absoluteDir,
      onlyFiles: true,
      followSymbolicLinks: false,
    });
    if (filepaths.length === 0) {
      this.context.stdout.write('Artifacts are empty');
      this.context.stdout.write('\n');
      return 0;
    }

    const totalNo = filepaths.length;
    this.context.stdout.write(`Found ${totalNo} file(s) to spread\n\n`);

    for (let i = 0; i < totalNo; i += 1) {
      const no = i + 1;
      const filepath = filepaths[i]!;
      const artifactFile = config.files.find(file => {
        const matches = minimatch(filepath, file.source);
        return matches;
      });
      if (artifactFile == null) {
        this.context.stderr.write(`[${no}/${totalNo}] Skip because file is not one of artifacts: "${filepath}"\n`);
        continue;
      }
      const src = path.join(config.absoluteDir, filepath);
      const dest = path.join(rootDir, artifactFile.dist, path.basename(src));
      this.context.stdout.write(`[${no}/${totalNo}] Copy "${filepath}" file into "${artifactFile.dist}"\n`);
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
