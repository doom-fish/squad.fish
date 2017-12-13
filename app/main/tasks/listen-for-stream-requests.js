const { ipcMain } = require('electron');
const ip = require('ip');
const CacheChunkStore = require('cache-chunk-store');
const FSChunkStore = require('fs-chunk-store'); // any chunk store will work
const { promisify } = require('util');
const fs = require('fs');
const torrentStream = require('torrent-stream');
const { RarFilesPackage } = require('rar-stream');
const readFile = promisify(fs.readFile);
const { connectToRenderObservable } = require('../utils/ipc');

const streamServer = require('../services/stream-server');
let engine;
let server;

const hasRarFiles = files =>
  files.filter(({ name }) => name.endsWith('rar')).length >= 1;
const progress = (message, data) => ({
  message,
  data,
  type: 'progress',
});
const response = data => ({ data, type: 'response' });

const getMediaFile = async (files, observer) => {
  if (hasRarFiles(files)) {
    observer.next(progress('parsing-rar-files'));

    files = files.filter(
      file =>
        !file.path
          .split('/')
          .map(part => part.toLowerCase())
          .includes('subs')
    );

    const rarPackage = new RarFilesPackage(files);
    rarPackage.on('parsing-start', bundle =>
      observer.next(progress('parsing-rar-files', bundle))
    );
    rarPackage.on('file-parsed', file =>
      observer.next(progress(`file-parsed: ${file.name}`))
    );
    const rarInnerFiles = await rarPackage.parse();
    files = [...files, ...rarInnerFiles];
  } else {
    observer.next(progress('is-plain'));
  }

  files.sort((a, b) => b.length - a.length);

  return files[0];
};

module.exports = app =>
  connectToRenderObservable('listen-for-stream', async observer => {
    observer.next(progress('waiting for torrent file', filePath));
    app.on('open-file', async (event, filePath) => {
      const torrentFile = await readFile(filePath);
      observer.next(progress('download-torrent-file-done', filePath));

      if (engine) {
        engine.destroy();
      }

      observer.next(progress('get-torrent-metadata'));
      engine = torrentStream(torrentFile, {
        verify: false,
        dht: false,
      });

      engine.on('ready', async () => {
        try {
          observer.next(progress('get-torrent-metadata-done'));
          observer.next(progress('get-media-file'));
          const mediaFile = await getMediaFile(engine.files, observer);
          observer.next(progress('get-media-file-done'));

          if (server) {
            server.destroy();
          }

          observer.next(progress('start-media-server'));
          server = streamServer(mediaFile);

          server.listen(0, () => {
            observer.next(
              response(`http://${ip.address()}:${server.address().port}`)
            );
            observer.complete();
          });
        } catch (error) {
          console.error(error);
          observer.error(error);
        }
      });
    });
  });
