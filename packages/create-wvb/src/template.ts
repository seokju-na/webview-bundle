import { createHash } from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import Mustache from 'mustache';
import { isYarnBerry, type PackageManager, runCommand } from './package-manager.js';

interface TemplateContext {
  nativeAppName: string;
  nativePackageSegment: string;
  nativeIdentifier: string;
  installCommand: string;
  devCommand: string;
  buildCommand: string;
  buildMainCommand: string;
  buildPreloadCommand: string;
}

export function createTemplateContext(
  projectName: string,
  manager: PackageManager
): TemplateContext {
  const words =
    projectName
      .normalize('NFKD')
      .replace(/\p{Diacritic}/gu, '')
      .match(/[a-zA-Z0-9]+/g) ?? [];
  let nativeAppName = words.map(word => word[0]!.toUpperCase() + word.slice(1)).join('');
  if (nativeAppName === '') {
    nativeAppName = `App${createHash('sha256').update(projectName).digest('hex').slice(0, 8)}`;
  }
  if (/^\d/.test(nativeAppName)) {
    nativeAppName = `App${nativeAppName}`;
  }
  // A prefix also prevents Java/Kotlin keywords from becoming a package segment.
  const nativePackageSegment = `app${nativeAppName.toLowerCase()}`;
  return {
    nativeAppName,
    nativePackageSegment,
    nativeIdentifier: `com.example.${nativePackageSegment}`,
    installCommand: `${manager.name} install`,
    devCommand: runCommand(manager, 'dev'),
    buildCommand: runCommand(manager, 'build'),
    buildMainCommand: runCommand(manager, 'build:main'),
    buildPreloadCommand: runCommand(manager, 'build:preload'),
  };
}

export function readTemplate(
  templateDir: string,
  context: TemplateContext,
  manager: PackageManager
): Map<string, Buffer> {
  const files = new Map<string, Buffer>();
  const renames: Record<string, string> = {
    _gitignore: '.gitignore',
    '_yarnrc.yml': '.yarnrc.yml',
    _npmrc: '.npmrc',
  };

  function readDirectory(directory: string, relativeDir: string): void {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      if (entry.name === '_yarnrc.yml' && !isYarnBerry(manager)) {
        continue;
      }
      if (entry.name === '_npmrc' && manager.name !== 'pnpm') {
        continue;
      }
      const source = path.join(directory, entry.name);
      const renderedName = Mustache.render(entry.name, context).replace(/\.mustache$/, '');
      const relative = path.join(relativeDir, renames[renderedName] ?? renderedName);
      if (entry.isDirectory()) {
        readDirectory(source, relative);
        continue;
      }
      if (entry.name.endsWith('.mustache')) {
        const content = Mustache.render(fs.readFileSync(source, 'utf8'), context);
        files.set(relative, Buffer.from(content));
      } else {
        files.set(relative, fs.readFileSync(source));
      }
    }
  }

  readDirectory(templateDir, '');
  return files;
}
