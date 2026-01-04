import { Command } from 'clipanion';
import { ColorOption, configureColor } from '../console.js';
import { configureLogger, getLogger, type Logger, LogLevelOption, LogVerboseOption } from '../log.js';

export abstract class BaseCommand extends Command {
  abstract readonly name: string;

  readonly color = ColorOption;
  readonly logLevel = LogLevelOption;
  readonly logVerbose = LogVerboseOption;

  private _logger: Logger | null = null;
  protected get logger(): Logger {
    if (this._logger == null) {
      throw new Error('Should configure logger before use');
    }
    return this._logger;
  }

  abstract run(): Promise<number | void>;

  async execute() {
    configureColor(this.color);
    await configureLogger({
      level: this.logLevel,
      verbose: this.logVerbose,
    });
    this._logger = getLogger(this.name);
    try {
      return await this.run();
    } catch (error) {
      this._logger.error(`"${this.name}" command failed with error: {error}`, { error });
      return 1;
    }
  }
}
