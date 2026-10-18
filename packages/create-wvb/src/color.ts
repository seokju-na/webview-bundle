const isColorEnabled = Boolean(
  process.stdout.isTTY &&
    process.stdout.hasColors?.() === true &&
    process.env.TERM !== 'dumb' &&
    process.env.NO_COLOR == null &&
    process.env.NODE_DISABLE_COLORS == null &&
    process.env.FORCE_COLOR !== '0'
);

if (!isColorEnabled) {
  process.env.NO_COLOR = '1';
  delete process.env.FORCE_COLOR;
}

export function color(format: (val: string) => string, text: string): string {
  if (isColorEnabled) {
    return format(text);
  }
  return text;
}
