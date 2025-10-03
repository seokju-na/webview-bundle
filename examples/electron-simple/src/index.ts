import { bundleProtocol, localProtocol, wvb } from '@webview-bundle/electron';
import { app, BrowserWindow } from 'electron';

wvb({
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
