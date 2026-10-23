import path from 'node:path';
import { app, BrowserWindow } from 'electron';
import { bundleProtocol, webviewBundle } from '@wvb/electron';

const wvb = webviewBundle({
  protocols: [
    bundleProtocol('app'),
  ],
});

wvb.ready().then(async () => {
  const win = new BrowserWindow({
    webPreferences: {
      contextIsolation: true,
      nodeIntegration: false,
      preload: path.join(__dirname, 'preload.js'),
    },
  });
  await win.loadURL('app://app.wvb/');
});
