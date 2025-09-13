import { Buffer } from 'node:buffer';
import fs from 'node:fs/promises';
import { promisify } from 'node:util';
import { deflate } from 'node:zlib';
import { findAllFixtureFiles, listFixtures } from '@benchmark/tools';
import { BundleBuilder } from '@webview-bundle/node-binding';
import { bench, describe } from 'vitest';
import { ZipFile } from 'yazl';

const deflateAsync = promisify(deflate);

function streamToBuffer(stream: NodeJS.ReadableStream): Promise<Buffer> {
  return new Promise<Buffer>((resolve, reject) => {
    const chunks: any[] = [];

    stream.on('data', chunk => chunks.push(chunk));
    stream.on('end', () => resolve(Buffer.concat(chunks)));
    stream.on('error', err => reject(err));
  });
}

for (const fixture of listFixtures()) {
  const files = await findAllFixtureFiles(fixture.name);

  describe(`create ${fixture.name}`, () => {
    bench(
      'yazl',
      async () => {
        const zip = new ZipFile();
        for (const file of files) {
          zip.addFile(file.absolutePath, file.path, { compress: false });
        }
        zip.end();
        await streamToBuffer(zip.outputStream);
      },
      { iterations: 100 }
    );

    bench(
      'yazl (compressed)',
      async () => {
        const zip = new ZipFile();
        for (const file of files) {
          zip.addFile(file.absolutePath, file.path, { compress: true });
        }
        zip.end();
        await streamToBuffer(zip.outputStream);
      },
      { iterations: 100 }
    );

    bench(
      'zlib',
      async () => {
        for (const file of files) {
          const data = await fs.readFile(file.absolutePath);
          await deflateAsync(data, { level: 1 });
        }
      },
      { iterations: 100 }
    );

    bench(
      'webview-bundle',
      async () => {
        const builder = new BundleBuilder();
        for (const file of files) {
          const data = await fs.readFile(file.absolutePath);
          builder.insertEntry(`/${file.path}`, data);
        }
        builder.build();
      },
      { iterations: 100 }
    );
  });
}
