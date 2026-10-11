import { Remote } from '@wvb/node';
import { Command, Option } from 'clipanion';
import { resolveConfig } from '../../config.js';
import { BaseCommand } from '../base.js';

export class RemoteListCommand extends BaseCommand {
  readonly name = 'remote-list';

  static paths = [
    ['remote', 'list'],
    ['remote', 'ls'],
  ];
  static usage = Command.Usage({
    description: 'List all Webview Bundles available on remote server.',
    details: `This command retrieves and displays a list of all Webview Bundles
stored on the remote server.

**Use Cases:**
  - Discover available bundles on a remote server
  - Audit deployed bundles across environments
  - Find bundle names for use with other commands
  - Inventory check before cleanup or migration

**Output Format:**
  The bundle list is displayed as JSON, making it easy to:
    - Parse in scripts and CI pipelines
    - Pipe to tools like \`jq\` for filtering
    - Integrate with monitoring and alerting systems

**Aliases:**
  This command can be invoked as either \`remote list\` or \`remote ls\`.
    `,
    examples: [
      ['List all bundles on a server', '$0 remote ls --base-url https://cdn.example.com'],
      ['List bundles using base url from config', '$0 remote list'],
      [
        'List and filter with jq',
        '$0 remote ls --base-url https://cdn.example.com | jq ".[].name"',
      ],
    ],
  });

  readonly baseUrl = Option.String('--base-url', {
    description: 'Base url of remote server.',
  });
  readonly channel = Option.String('--channel', {
    description:
      'Release channel to manage and distribute different stability versions. (e.g. "beta", "alpha")',
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
    const baseUrl = this.baseUrl ?? config.remote?.baseUrl;
    if (baseUrl == null) {
      this.logger.error('"baseUrl" is required for remote operations.');
      return 1;
    }
    const remote = new Remote({ baseUrl });
    const bundles = (await remote.getUpdate({ channel: this.channel }))?.update.bundles ?? [];
    this.logger.info(`Remote Webview Bundles:`);
    console.log(JSON.stringify(bundles, null, 2));
  }
}
