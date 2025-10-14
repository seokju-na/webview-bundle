import path from 'node:path';
import { bundleProtocol, localProtocol, wvb } from '@webview-bundle/electron';
import { app, BrowserWindow } from 'electron';

wvb({
  remote: {
    endpoint: 'https://dkff9jdtl2tsb.cloudfront.net',
  },
  protocols: [
    localProtocol('app-local', {
      hosts: {
        'wvb.dev': 'http://localhost:4312',
      },
    }),
    bundleProtocol('app', {
      onError: e => console.error(e),
    }),
  ],
});

async function createWindow() {
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: path.join(import.meta.dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  await mainWindow.loadURL('app://out.wvb');
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
