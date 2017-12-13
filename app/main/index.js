const { app } = require('electron');
const enableMPVPlugin = require('./tasks/enable-mpv-plugin');
const createMainWindow = require('./tasks/create-main-window');

const listenForStreamRequests = require('./tasks/listen-for-stream-requests');
const listenForChromecastRequests = require('./tasks/listen-for-chromecast-requests');

enableMPVPlugin(app);
createMainWindow(app);
listenForChromecastRequests();
listenForStreamRequests(app);

app.on('window-all-closed', () => app.quit());
