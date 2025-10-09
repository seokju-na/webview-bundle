import { rolldown } from 'rolldown';
import path from 'node:path';
import fs from 'node:fs/promises';

const bundle = await rolldown({
  input: './lambda/origin-request.ts',
  platform: 'node',
  external: () => false,
  treeshake: true,
  transform: {
    target: 'node22',
  },
  define: {
    __CONFIG__: JSON.stringify({
      bucketName: 'test',
    })
  }
});

const { output: outputs } = await bundle.generate({
  format: 'esm',
  sourcemap: true,
  minify: false,
  entryFileNames: '[name].mjs',
  chunkFileNames: '[name]-[hash].mjs',
});

for (const output of outputs) {
  if (output.type === 'chunk') {
    const filepath = path.join(
      process.cwd(),
      'dist-lambda',
      output.fileName,
    );
    await fs.mkdir(path.dirname(filepath), { recursive: true });
    await fs.writeFile(filepath, output.code, 'utf8');
    if (output.map != null) {
      const filepath = path.join(
        process.cwd(),
        'dist-lambda',
        output.sourcemapFileName,
      );
      await fs.mkdir(path.dirname(filepath), { recursive: true });
      await fs.writeFile(
        filepath,
        output.map.toString(),
        'utf8'
      );
    }
  }
}
