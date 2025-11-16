export function formatByteLength(byteLength: number) {
  if (byteLength === 0) {
    return '0 B';
  }
  const units = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
  const base = 1024;

  const unitIdx = Math.floor(Math.log(byteLength) / Math.log(base));
  const size = byteLength / base ** unitIdx;

  const formattedSize = parseFloat(size.toFixed(2));

  return `${formattedSize} ${units[unitIdx]}`;
}
