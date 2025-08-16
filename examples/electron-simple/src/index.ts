import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { webviewBundle } from '@webview-bundle/electron';
import { app, BrowserWindow } from 'electron';

const dirname = path.dirname(fileURLToPath(import.meta.url));
// const loader = FSLoader.fromDir(path.join(dirname, '../'));

webviewBundle({
  protocol: {
    scheme: 'app',
    privileges: {},
  },
  updater: {
    remotesBaseUrl: 'https://wvb.mycdn.com',
  },
});

// registerProtocol({ scheme: 'app', loader });

async function createWindow() {
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
  });

  await mainWindow.loadURL('app://bundle');
  mainWindow.webContents.openDevTools();
}

app.on('ready', () => {
  createWindow();
});
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});
