import path from 'node:path';
import { BrowserWindow, app } from 'electron';
import serve from 'electron-serve';

serve({
  scheme: 'app',
  directory: path.resolve(__dirname, '../../../fixtures/next/out'),
});

let mainWindow: BrowserWindow;

(async () => {
  await app.whenReady();
  mainWindow = new BrowserWindow();
  await mainWindow.loadURL('app://-');
})();
