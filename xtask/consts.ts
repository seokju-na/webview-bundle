import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const ROOT_DIR = path.join(path.dirname(fileURLToPath(import.meta.url)), '..');
