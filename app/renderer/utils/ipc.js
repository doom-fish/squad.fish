import { ipcRenderer } from 'electron';
import { Subject, Observable } from 'rxjs';
import newRequestId from 'uuid/v1';

class ErrorWrapper extends Error {
  constructor(innerError) {
    super(`Error in main process: ${innerError.message}`);
    this.stack = innerError.stack;
  }
}

export const ipcObservable = (topic, data) =>
  Observable.create(observer => {
    const onMessage = (event, message) => observer.next(message);
    const onError = (event, error) => observer.error(new ErrorWrapper(error));
    const onComplete = event => observer.complete();

    ipcRenderer.on(topic, onMessage);
    ipcRenderer.on(`${topic}-error`, onError);
    ipcRenderer.once(`${topic}-complete`, onComplete);
    ipcRenderer.send(topic, data);
    return () => {
      ipcRenderer.removeListener(topic, onMessage);
      ipcRenderer.removeListener(`${topic}-error`, onError);
    };
  });

export const ipcRequest = (topic, data) => {
  const replySubject = new Subject();
  const requestId = newRequestId();
  const onError = (event, error) => replySubject.error(new ErrorWrapper(error));
  const onResponse = (event, response) => {
    replySubject.next(response);
    ipcRenderer.removeListener(`${topic}-${requestId}-error`, onError);
    replySubject.complete();
  };
  ipcRenderer.once(`${topic}-${requestId}-error`, onError);
  ipcRenderer.once(`${topic}-${requestId}`, onResponse);

  ipcRenderer.send(topic, {
    requestId,
    data,
  });

  return replySubject;
};
