import CI from 'ci-info';
import { Option } from 'clipanion';
import kleur from 'kleur';
import supportsColor from 'supports-color';
import { isEnum } from 'typanion';

export const ColorOption = Option.String('--color,-C', 'auto', {
  validator: isEnum(['off', 'on', 'auto'] as const),
  description: 'Set the color mode for output.',
  env: 'COLOR',
});

export function configureColor(val: typeof ColorOption) {
  switch (val) {
    case 'off':
      kleur.enabled = false;
      break;
    case 'on':
      kleur.enabled = true;
      break;
    case 'auto':
      kleur.enabled = CI.GITHUB_ACTIONS ? true : supportsColor.stdout !== false && supportsColor.stdout.level > 0;
      break;
  }
}

export const colors = {
  debug: (msg: string | number) => kleur.gray(msg),
  info: (msg: string | number) => kleur.white(msg),
  warn: (msg: string | number) => kleur.yellow(msg),
  error: (msg: string | number) => kleur.red(msg),
  success: (msg: string | number) => kleur.green(msg),
  bytes: (msg: string | number) => kleur.gray(msg),
  bold: (msg: string | number) => kleur.bold(msg),
  underline: (msg: string | number) => kleur.underline(msg),
};
