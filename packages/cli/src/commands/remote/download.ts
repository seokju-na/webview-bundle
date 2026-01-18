import { Presets, SingleBar } from 'cli-progress';
import { Command, Option } from 'clipanion';
import { BaseCommand } from '../base.js';

export class RemoteDownloadCommand extends BaseCommand {
  readonly name = 'remote-download';

  static paths = [['remote', 'download']];
  static usage = Command.Usage({
    description: '',
  });

  readonly bundleName = Option.String({
    name: 'BUNDLE',
    required: false,
  });
  readonly version = Option.String('--version,-V', {
    description: '',
  });
  readonly endpoint = Option.String('--endpoint,-E', {
    description: 'Endpoint of remote server.',
  });
  readonly configFile = Option.String('--config,-C', {
    description: 'Config file path',
  });
  readonly cwd = Option.String('--cwd', {
    description: 'Current working directory.',
  });

  async run() {
    const progress = new SingleBar(
      {
        clearOnComplete: true,
        format: '',
        hideCursor: true,
        gracefulExit: true,
      },
      Presets.shades_classic
    );
  }
}
