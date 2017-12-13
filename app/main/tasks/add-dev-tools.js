module.exports = async () => {
  if (process.env.NODE_ENV !== 'production') {
    const {
      default: installExtension,
      REACT_DEVELOPER_TOOLS,
      ANGULARJS_BATARANG,
      REDUX_DEVTOOLS,
      REACT_PERF,
    } = require('electron-devtools-installer');
    await installExtension(REDUX_DEVTOOLS);
    await installExtension(REACT_DEVELOPER_TOOLS);
    await installExtension(REACT_PERF);
  }
};
