const chromecasts = require('chromecasts');

const {
  connectToRenderObservable,
  listenForIpcRequests,
} = require('../utils/ipc');

setTimeout(() => chromecastList.update(), 60000);
const chromecastList = chromecasts();

const serializePlayers = players =>
  players.map(({ domain, host, name }) => ({ domain, host, name }));

const reportDiscoveredChromecasts = async observer => {
  chromecastList.update();
  observer.next(serializePlayers(chromecastList.players));

  chromecastList.on('update', () =>
    observer.next(serializePlayers(chromecastList.players))
  );
};

const interceptCastTpDeviceRequests = async (requestedDevice, streamUrl) => {
  const [player] = chromecastList.players.filter(
    player =>
      player.host === requestedDevice.host &&
      player.name === requestedDevice.name &&
      player.domain === requestedDevice.domain
  );
  if (!player) {
    throw new Error(
      `Device: ${requestedDevice.name} not found in your network`
    );
  }
  player.play(stremaUrl, { type: 'video/mp4' });
};

module.exports = () => {
  listenForIpcRequests('cast-to-device', interceptCastTpDeviceRequests);
  connectToRenderObservable(
    'discovered-chromecasts',
    reportDiscoveredChromecasts
  );
};
