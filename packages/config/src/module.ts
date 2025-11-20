import { builtinModules } from 'node:module';

export function isNodeBuiltin(mod: string): boolean {
  if (mod.startsWith('node:')) {
    return true;
  }
  return builtinModules.some(x => x === mod);
}
