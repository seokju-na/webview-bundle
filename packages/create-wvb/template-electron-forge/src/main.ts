import path from 'node:path';
import { bundleProtocol, webviewBundle } from '@wvb/electron';
import { app, BrowserWindow } from 'electron';

const wvb = webviewBundle({
  source: app.isPackaged
    ? undefined
    : { builtinDir: path.join(app.getAppPath(), '.wvb', 'builtin', 'bundles') },
  protocols: [bundleProtocol('bundle')],
});

async function createWindow() {
  await wvb.ready();
  const window = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      contextIsolation: true,
      sandbox: true,
      preload: path.join(import.meta.dirname, 'preload.cjs'),
    },
  });
  await window.loadURL('bundle://app.wvb/');
}

createWindow().catch(error => {
  console.error(error);
  app.quit();
});
app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    void createWindow();
  }
});
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
