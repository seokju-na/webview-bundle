import CI from 'ci-info';
import { Option } from 'clipanion';
import kleur from 'kleur';
import supportsColor from 'supports-color';
import { isEnum } from 'typanion';

export const ColorOption = Option.String('--color,-C', 'auto', {
  validator: isEnum(['off', 'on', 'auto'] as const),
  description: 'Set the color mode for output.',
});
export type ColorOptionValue = typeof ColorOption;

export function applyColorOption(val: ColorOptionValue) {
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
