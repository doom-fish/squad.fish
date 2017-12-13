const path = require('path');

let resourcesPath = process.resourcesPath;
if (process.env.NODE_ENV === 'development') {
  resourcesPath = path.resolve(__dirname, '../../');
}
const mpvPluginID = () => {
  const PLUGIN_MIME_TYPE = 'application/x-mpvjs';
  const pluginDir = path.join(resourcesPath, '/mpv/mpvjs.node');
  let pluginPath = path.relative(process.cwd(), pluginDir);
  return `${pluginPath};${PLUGIN_MIME_TYPE}`;
};

module.exports = app => {
  app.commandLine.appendSwitch('ignore-gpu-blacklist');
  app.commandLine.appendSwitch('register-pepper-plugins', mpvPluginID());
};
