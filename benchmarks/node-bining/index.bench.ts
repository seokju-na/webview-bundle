import fs from 'node:fs/promises';
import { create } from '@webview-bundle/node-binding';
import { findAllFixtureFiles, listFixtures } from 'benchmark-tools';
import { bench, describe } from 'vitest';
import { ZipFile } from 'yazl';

function streamToBuffer(stream: NodeJS.ReadableStream) {
  return new Promise((resolve, reject) => {
    const chunks: any[] = [];

    stream.on('data', chunk => chunks.push(chunk));
    stream.on('end', () => resolve(Buffer.concat(chunks)));
    stream.on('error', err => reject(err));
  });
}

for (const fixture of listFixtures()) {
  describe(`create - ${fixture.name}`, () => {
    const title = `create - ${fixture.name}`;
    bench(
      `${title}/zip`,
      async () => {
        const files = await findAllFixtureFiles(fixture.name);
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
      `${title}/zip (compress)`,
      async () => {
        const files = await findAllFixtureFiles(fixture.name);
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
      `${title}/webview-bundle`,
      async () => {
        const files = await findAllFixtureFiles(fixture.name);
        await create(
          await Promise.all(
            files.map(async file => ({
              path: file.path,
              data: await fs.readFile(file.absolutePath),
            }))
          )
        );
      },
      { iterations: 100 }
    );
  });
}
