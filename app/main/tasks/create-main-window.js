const { BrowserWindow } = require('electron');
const createMenu = require('./create-menu');
const isDevelopment = process.env.NODE_ENV === 'development';
const addDevTools = require('./add-dev-tools');

const entryDocumentUrl = () =>
  isDevelopment
    ? 'http://localhost:3000?react_perf'
    : `file://${__dirname}/../../dist/index.html`;

const windowTitle = isDevelopment ? '[DEVELOPMENT] Spectrum' : 'Spectrum';
module.exports = app =>
  app.on('ready', () => {
    const win = new BrowserWindow({
      width: 1280,
      height: 574,
      show: false,
      minHeight: 300,
      minWidth: 800,
      backgroundColor: '#000000',
      autoHideMenuBar: true,
      title: windowTitle,
      titleBarStyle: 'hidden',
      webPreferences: {
        experimentalFeatures: true,
        plugins: true,
      },
    });
    addDevTools();
    win.setMenu(createMenu(app));
    win.loadURL(entryDocumentUrl());
    win.once('ready-to-show', () => win.show());
  });
