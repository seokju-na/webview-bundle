import path from 'node:path';
import { FSLoader, protocolHandler } from '@webview-bundle/electron';
import { BrowserWindow, app, protocol } from 'electron';

protocol.registerSchemesAsPrivileged([
  {
    scheme: 'app',
    privileges: {
      standard: true,
      secure: true,
      supportFetchAPI: true,
      bypassCSP: true,
      corsEnabled: true,
      codeCache: true,
    },
  },
]);

async function createWindow() {
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
  });

  await mainWindow.loadURL('app://bundle');
  mainWindow.webContents.openDevTools();
}

const handler = protocolHandler({
  loader: FSLoader.fromDir(path.join(__dirname, '../')),
});

app.on('ready', () => {
  protocol.handle('app', handler);
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
