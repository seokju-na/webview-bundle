import { Command } from 'clipanion';
import { loadConfigFile } from '../config.js';

export class TestCommand extends Command {
  static paths = [['test']];

  async execute() {
    await loadConfigFile();
  }
}
