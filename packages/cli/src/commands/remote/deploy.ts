import { Command, Option } from 'clipanion';
import { resolveConfig } from '../../config.js';
import { c } from '../../console.js';
import { BaseCommand } from '../base.js';

export class RemoteDeployCommand extends BaseCommand {
  readonly name = 'remote-deploy';

  static paths = [['remote', 'deploy']];
  static usage = Command.Usage({
    description: 'Deploy Webview Bundle to the remote server.',
    details: `
      This command deploys a previously uploaded Webview Bundle version,
      making it available to clients via the configured deployer.

      **Upload vs Deploy:**
        - \`remote upload\`: Transfers the bundle file to remote storage
        - \`remote deploy\`: Activates a specific version for client consumption

      This separation allows you to upload multiple versions and then
      selectively deploy them, enabling staged rollouts and instant rollbacks.

      **Version Resolution:**
        - If \`VERSION\` argument is provided, that version is deployed
        - Otherwise, falls back to \`version\` field in package.json

      **Channel:**
        Channels allow you to deploy different versions to different audiences.
        Common use cases include:
          - \`stable\` / \`beta\` / \`canary\` release tracks
          - \`internal\` for team testing before public release
          - A/B testing with percentage-based routing

        If no channel is specified, the bundle is deployed to the default channel.
    `,
    examples: [
      ['Deploy latest version from package.json', '$0 remote deploy myapp'],
      ['Deploy a specific version', '$0 remote deploy myapp 1.2.0'],
      ['Deploy to a specific channel', '$0 remote deploy myapp 1.2.0 --channel beta'],
    ],
  });

  readonly bundleName = Option.String({
    name: 'BUNDLE',
    required: false,
  });
  readonly version = Option.String({
    name: 'VERSION',
  });
  readonly channel = Option.String('--channel,-C', {
    description: 'Release channel to manage and distribute different stability versions. (e.g. "beta", "alpha")',
  });
  readonly configFile = Option.String('--config,-C', {
    description: 'Path to the config file.',
  });
  readonly cwd = Option.String('--cwd', {
    description: 'Set the working directory for resolving paths. [Default: process.cwd()]',
  });

  async run() {
    const config = await resolveConfig({
      root: this.cwd,
      configFile: this.configFile,
    });
    const bundleName = this.bundleName ?? config.remote?.bundleName;
    if (bundleName == null) {
      this.logger.error('"bundleName" is required for remote operations.');
      return 1;
    }
    const version = this.version ?? config.packageJson?.version;
    if (version == null) {
      this.logger.error('Cannot get version of this Webview Bundle.');
      return 1;
    }
    if (config.remote?.deployer == null) {
      this.logger.error(
        'Cannot get "deployer" from remote config. Make sure the "deployer" is defined in remote config.'
      );
      return 1;
    }
    await config.remote.deployer.deploy({
      bundleName,
      version,
      channel: this.channel,
    });
    this.logger.info(`Remote Webview Bundle deployed: ${c.info(bundleName)}`);
    this.logger.info(`  Version: ${c.bold(c.info(version))}`);
    if (this.channel != null) {
      this.logger.info(`  Channel: ${c.bold(c.info(this.channel))}`);
    }
  }
}
