import { Command } from 'clipanion';
import { VersionedFile } from '../versioned-file.ts';

export class ReleaseCommand extends Command {
  static paths = [['release']];

  async execute() {
    const file = await VersionedFile.load('crates/webview-bundle-cli/Cargo.toml');
    console.log(file.version.toString());
    file.bumpVersion({ type: 'major' });
    console.log(await file.write());
  }
}
