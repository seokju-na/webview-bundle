import fs from 'node:fs/promises';
import path from 'node:path';
import type { Writable } from 'node:stream';
import { partition } from 'es-toolkit';
import {
  sys,
  type CompilerOptions,
  DiagnosticCategory,
  ModuleKind,
  ModuleResolutionKind,
  ScriptTarget,
  createCompilerHost,
  createProgram,
  formatDiagnosticsWithColorAndContext,
  parseJsonConfigFileContent,
} from 'typescript';

interface Options {
  cwd?: string;
  stdout?: Writable;
  stderr?: Writable;
}

export async function buildDTS(
  src: string,
  outdir: string,
  { cwd = process.cwd(), stdout = process.stdout, stderr = process.stderr }: Options = {}
) {
  const compilerOptions: CompilerOptions = {
    target: ScriptTarget.ESNext,
    module: ModuleKind.ESNext,
    moduleResolution: ModuleResolutionKind.NodeNext,
    allowJs: true,
    esModuleInterop: true,
    allowSyntheticDefaultImports: true,
    composite: false,
    incremental: false,
    skipLibCheck: true,
    declaration: true,
    emitDeclarationOnly: true,
    noEmit: false,
  };
  const config = parseJsonConfigFileContent({ compilerOptions }, sys, cwd);
  const srcDir = path.join(cwd, src);
  const files = config.fileNames.filter(x => x.startsWith(srcDir));

  stdout.write(`Build DTS (${outdir}):\n`);
  for (const file of files) {
    const relativePath = path.relative(cwd, file);
    stdout.write(`  ${relativePath}\n`);
  }

  const host = createCompilerHost(config.options);
  const program = createProgram(files, config.options, host);

  const outputs = new Map<string, Buffer>();
  const result = program.emit(
    undefined,
    (filename, content) => {
      outputs.set(filename, Buffer.from(content, 'utf8'));
    },
    undefined,
    true
  );
  const [errors, others] = partition(result.diagnostics, x => x.category === DiagnosticCategory.Error);
  if (errors.length > 0) {
    stderr.write(formatDiagnosticsWithColorAndContext(errors, host));
    stderr.write('\n');
    throw new Error('build types failed');
  }
  if (others.length > 0) {
    stdout.write(formatDiagnosticsWithColorAndContext(others, host));
    stdout.write('\n');
  }
  for (const [absoluteFilepath, content] of outputs.entries()) {
    const filepath = path.join(cwd, outdir, path.relative(srcDir, absoluteFilepath));
    await fs.writeFile(filepath, content, 'utf8');
  }
}
