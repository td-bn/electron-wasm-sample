import { app, BrowserWindow, ipcMain } from 'electron';
// TODO: Why doesn't this work?
import { greet, Wollet } from '../client-wasm/pkg/client_wasm'
import * as path from 'path';

const createWindow = () => {
  const win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
    },
  });

  ipcMain.handle('ping', () => 'pong');
  ipcMain.handle('greet', (_e, s: string) => {
    return greet(s);
  });
  console.log(__dirname);
  win.loadURL('http://localhost:3002') // Locally running CRA instance
};

app.whenReady().then(() => {
  createWindow();

  const m = 'faith fossil mushroom quiz bunker can palm ghost obey advance dismiss toddler';
  let wallet = new Wollet(m);
  console.log(wallet);

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
