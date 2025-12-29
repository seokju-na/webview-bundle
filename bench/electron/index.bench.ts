import { _electron as electron } from 'playwright';
import { type BenchFunction, bench, describe } from 'vitest';

function startup(mainFilePath: string): BenchFunction {
  return async () => {
    const app = await electron.launch({ args: [mainFilePath] });
    const win = await app.firstWindow();
    await win.waitForLoadState();
    app.close();
    await app.waitForEvent('close', { timeout: 5_000 });
  };
}

describe('startup', () => {
  bench('fs', startup('./dist/fs/main.mjs'), { iterations: 10 });
  bench('webview-bundle', startup('./dist/wvb/main.mjs'), { iterations: 10 });
});

function navigation(mainFilePath: string): BenchFunction {
  return async () => {
    const app = await electron.launch({ args: [mainFilePath] });
    const win = await app.firstWindow();
    await win.waitForLoadState();
    await win.getByRole('link', { name: /DEMO: Go to category page \(PLP\)/ }).click();
    await win.getByText(/Page 1/).waitFor({ state: 'visible' });
    await win.getByRole('link', { name: '2' }).click();
    await win.getByText(/Page 2/).waitFor({ state: 'visible' });
    await win.getByRole('link', { name: '3' }).click();
    await win.getByText(/Page 3/).waitFor({ state: 'visible' });
    await win.getByRole('link', { name: '10' }).click();
    await win.getByText(/Page 10/).waitFor({ state: 'visible' });
    app.close();
    await app.waitForEvent('close', { timeout: 5_000 });
  };
}

describe('navigation', () => {
  bench('fs', navigation('./dist/fs/main.mjs'), { iterations: 10 });
  bench('webview-bundle', navigation('./dist/wvb/main.mjs'), { iterations: 10 });
});
