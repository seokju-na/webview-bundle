import fs from 'node:fs';
import { zodToJsonSchema } from 'zod-to-json-schema';
import { ArtifactsConfigSchema } from './src/artifacts/config';
import { ReleaseConfigSchema } from './src/release/config';

const releasesConfigSchema = zodToJsonSchema(ReleaseConfigSchema);
const artifactsConfigSchema = zodToJsonSchema(ArtifactsConfigSchema);

fs.mkdirSync('./dist/schema', { recursive: true });
fs.writeFileSync('./dist/schema/releases.json', JSON.stringify(releasesConfigSchema, null, 2), 'utf8');
fs.writeFileSync('./dist/schema/artifacts.json', JSON.stringify(artifactsConfigSchema, null, 2), 'utf8');
