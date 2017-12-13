const { Observable } = require('rxjs');
const { ipcMain } = require('electron');
const newId = require('uuid/v4');

const connectToRenderObservable = (topic, listener) => {
  ipcMain.on(topic, async ({ sender }, request) => {
    const observable = Observable.create(async observer => {
      try {
        await listener(observer, request);
      } catch (error) {
        console.error(error);
        observer.error(error);
      }
    });

    observable.subscribe(
      message => sender.send(topic, message),
      error => sender.send(`${topic}-error`, error),
      complete => sender.send(`${topic}-complete`, complete)
    );
  });
};

const listenForIpcRequests = (topic, listener) => {
  ipcMain.on(topic, async ({ sender }, request) => {
    try {
      const response = await listener(request.data);
      sender.send(`${topic}-${request.requestId}`, response);
    } catch (error) {
      console.error(error);

      sender.send(`${topic}-${request.requestId}-error`, {
        message: error.message,
        stack: error.stack,
      });
    }
  });
};

module.exports = { connectToRenderObservable, listenForIpcRequests };
