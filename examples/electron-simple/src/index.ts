import path from 'node:path';
import { protocolHandler } from '@webview-bundle/electron';
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

  await mainWindow.loadURL('app://test/');
  mainWindow.webContents.openDevTools();
}

app.on('ready', () => {
  protocol.handle(
    'app',
    protocolHandler({
      bundleDir: path.join(__dirname, '../'),
    })
  );
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
