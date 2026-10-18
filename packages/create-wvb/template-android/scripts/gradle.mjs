import { fileURLToPath } from 'node:url';
import { exec } from 'es-toolkit/server';

const isWindows = process.platform === 'win32';
const wrapper = fileURLToPath(
  new URL(isWindows ? '../gradlew.bat' : '../gradlew', import.meta.url)
);
const command = isWindows ? `"${wrapper}"` : wrapper;
const result = await exec(command, process.argv.slice(2), {
  spawnOptions: { stdio: 'inherit', shell: isWindows },
  throwOnNonZeroExitCode: false,
});
process.exitCode = result.exitCode ?? 1;
