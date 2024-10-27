import fs from 'node:fs/promises';
import { zodToJsonSchema } from 'zod-to-json-schema';
import { ArtifactsConfigSchema } from './src/artifacts/config';
import { ReleaseConfigSchema } from './src/release/config';

const releasesConfigSchema = zodToJsonSchema(ReleaseConfigSchema);
const artifactsConfigSchema = zodToJsonSchema(ArtifactsConfigSchema);

await fs.mkdir('./dist/schema', { recursive: true });
await Promise.all([
  fs.writeFile('./dist/schema/releases.json', JSON.stringify(releasesConfigSchema, null, 2), 'utf8'),
  fs.writeFile('./dist/schema/artifacts.json', JSON.stringify(artifactsConfigSchema, null, 2), 'utf8'),
]);
