#!/usr/bin/env node
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import * as prompts from '@clack/prompts';
import { colors, ExecError } from 'es-toolkit/server';
import mri from 'mri';
import { Cancelled } from './cancelled.js';
import { color } from './color.js';
import {
  getPackageManager,
  installDependencies,
  type PackageManager,
  runCommand,
} from './package-manager.js';
import { createTemplateContext, readTemplate } from './template.js';

const templates = [
  { name: 'electron', label: 'Electron' },
  { name: 'electron-forge', label: 'Electron Forge' },
  { name: 'electron-builder', label: 'Electron Builder' },
  { name: 'tauri', label: 'Tauri' },
  { name: 'android', label: 'Android' },
  { name: 'ios', label: 'iOS' },
  { name: 'deno-desktop', label: 'Deno Desktop' },
];
const packageRoot = path.join(import.meta.dirname, '..');
const cwd = process.cwd();
const defaultTargetDir = 'wvb-project';
const helpMessage = `Usage: create-wvb [DIRECTORY] [OPTIONS]

Create a Webview Bundle project. Interactive when run in a terminal.

Options:
  -t, --template NAME                select a template (default: electron)
  -i, --install / --no-install        install dependencies after scaffolding
  --overwrite                       remove existing files except .git
  --interactive / --no-interactive   force interactive / non-interactive mode
  -h, --help                         display this help message

Templates:
${templates.map(template => `  ${color(colors.cyan, template.name)}`).join('\n')}

Example:
  yarn create wvb my-app --template electron-forge`;

function answer<T>(value: T): Exclude<T, symbol> {
  if (prompts.isCancel(value)) {
    throw new Cancelled();
  }
  return value as Exclude<T, symbol>;
}

function packageName(value: string): string {
  return (
    value
      .trim()
      .toLowerCase()
      .replace(/^[._]+/, '')
      .replace(/[^a-z\d~-]+/g, '-') || 'wvb-app'
  );
}

function isPackageName(value: string): boolean {
  return /^(?:@[a-z\d~][a-z\d._~-]*\/)?[a-z\d~-][a-z\d._~-]*$/.test(value);
}

function isSameOrChildPath(parent: string, child: string): boolean {
  const relative = path.relative(parent, child);
  return (
    relative === '' ||
    (relative !== '..' && !relative.startsWith(`..${path.sep}`) && !path.isAbsolute(relative))
  );
}

function validateTarget(root: string): void {
  if (
    fs.existsSync(root) &&
    (!fs.lstatSync(root).isDirectory() || fs.lstatSync(root).isSymbolicLink())
  ) {
    throw new Error('Target must be a directory, not a file or symbolic link.');
  }
  if (root === path.parse(root).root || root === os.homedir()) {
    throw new Error('Choose a project directory instead of the filesystem root or home directory.');
  }
  // Resolve existing ancestors as well, so paths through symlinks cannot erase the CLI or cwd.
  let ancestor = root;
  while (!fs.existsSync(ancestor)) {
    ancestor = path.dirname(ancestor);
  }
  const resolved = path.resolve(fs.realpathSync(ancestor), path.relative(ancestor, root));
  const installation = fs.realpathSync(packageRoot);
  if (isSameOrChildPath(installation, resolved)) {
    throw new Error('Target must be outside the create-wvb installation.');
  }
  const currentDirectory = fs.realpathSync(cwd);
  const containsInstallation = isSameOrChildPath(resolved, installation);
  const containsCurrentDirectory =
    resolved !== currentDirectory && isSameOrChildPath(resolved, currentDirectory);
  if (containsInstallation || containsCurrentDirectory) {
    throw new Error('Target must not contain the current directory or create-wvb installation.');
  }
}

async function init(): Promise<void> {
  const argv = mri<{
    template?: string;
    help?: boolean;
    overwrite?: boolean;
    install?: boolean;
    interactive?: boolean;
  }>(process.argv.slice(2), {
    boolean: ['help', 'overwrite', 'install', 'interactive'],
    alias: { h: 'help', t: 'template', i: 'install', overwrite: [], interactive: [] },
    string: ['template'],
    unknown: option => {
      throw new Error(`Unknown option: ${option}. Run create-wvb --help.`);
    },
  });
  if (argv.help === true) {
    console.log(helpMessage);
    return;
  }
  if (argv._.length > 1) {
    throw new Error('Specify only one project directory.');
  }
  const interactive = argv.interactive ?? (process.stdin.isTTY && process.stdout.isTTY);
  let targetDir = argv._[0] != null ? String(argv._[0]).trim() : undefined;
  if (targetDir === '') {
    throw new Error('Project directory cannot be empty.');
  }
  if (targetDir == null) {
    targetDir = answer(
      await prompts.text({
        message: 'Project name:',
        placeholder: defaultTargetDir,
        defaultValue: defaultTargetDir,
        validate: value => {
          if (value != null && value !== '' && value.trim() === '') {
            return 'Enter a project name.';
          }
        },
      })
    ).trim();
  }
  const root = path.resolve(cwd, targetDir);
  validateTarget(root);

  let template = argv.template;
  if (template != null && !templates.some(item => item.name === template)) {
    throw new Error(
      `Unknown template: ${argv.template}. Available templates: ${templates.map(item => item.name).join(', ')}`
    );
  }
  if (template == null) {
    template = answer(
      await prompts.select({
        message: 'Select a template:',
        options: templates.map(item => ({
          value: item.name,
          label: color(colors.cyan, item.label),
        })),
      })
    );
  }
  const templateDir = path.join(packageRoot, `template-${template}`);
  const manager: PackageManager =
    template === 'deno-desktop' ? { name: 'deno' } : getPackageManager();
  const context = createTemplateContext(path.basename(root), manager);
  const files = readTemplate(templateDir, context, manager);
  const packageContent = files.get('package.json');
  const pkg = packageContent != null ? JSON.parse(packageContent.toString()) : undefined;
  let name = path.basename(root);
  if (pkg != null && !isPackageName(name)) {
    name = interactive
      ? answer(
          await prompts.text({
            message: 'Package name:',
            placeholder: packageName(name),
            defaultValue: packageName(name),
            validate: value => {
              if (value != null && value !== '' && !isPackageName(value)) {
                return 'Invalid package.json name.';
              }
            },
          })
        )
      : packageName(name);
  }
  if (pkg != null) {
    pkg.name = name;
    files.set('package.json', Buffer.from(`${JSON.stringify(pkg, null, 2)}\n`));
  }

  const entries = fs.existsSync(root) ? fs.readdirSync(root).filter(file => file !== '.git') : [];
  let overwrite = argv.overwrite ?? false;
  if (entries.length > 0 && !overwrite) {
    if (!interactive || argv.overwrite === false) {
      throw new Error(
        `Target directory is not empty: ${root}. Use --overwrite to remove existing files.`
      );
    }
    overwrite = answer(
      await prompts.select({
        message: `Target directory "${targetDir}" is not empty. How would you like to proceed?`,
        options: [
          { label: 'Cancel operation', value: false },
          { label: 'Remove existing files and continue', value: true },
        ],
      })
    );
    if (!overwrite) {
      throw new Cancelled();
    }
  }
  const install =
    argv.install ??
    (interactive
      ? answer(
          await prompts.confirm({
            message: `Install dependencies with ${manager.name} now?`,
            initialValue: false,
          })
        )
      : false);

  if (overwrite) {
    for (const entry of entries) {
      fs.rmSync(path.join(root, entry), { recursive: true, force: true });
    }
  }
  fs.mkdirSync(root, { recursive: true });
  console.log(`\nScaffolding project in ${root}...`);
  for (const [file, content] of files) {
    const destination = path.join(root, file);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, content);
  }
  // Package archives may normalize file modes; Gradle's wrapper must remain executable.
  if (template === 'android') {
    fs.chmodSync(path.join(root, 'gradlew'), 0o755);
  }
  if (install) {
    console.log(`\nInstalling dependencies with ${manager.name}...`);
    await installDependencies(manager, root);
  }
  const relative = path.relative(cwd, root);
  const directory = relative !== '' ? `\n  cd ${JSON.stringify(relative)}` : '';
  const installInstruction = install ? '' : `\n  ${context.installCommand}`;
  console.log(
    color(
      colors.green,
      `\nDone. Now run:${directory}${installInstruction}\n  ${runCommand(manager, 'dev')}\n`
    )
  );
}

init().catch(error => {
  if (error instanceof Cancelled) {
    console.log('Operation cancelled.');
    process.exitCode = 1;
    return;
  }
  console.error(color(colors.red, error instanceof Error ? error.message : String(error)));
  const exitCode = error instanceof ExecError ? error.result.exitCode : null;
  process.exitCode = exitCode != null && exitCode !== 0 ? exitCode : 1;
});
