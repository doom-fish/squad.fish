//@flow
import React, { Element } from 'react';
import EventEmitter from 'events';
import { Observable } from 'rxjs';
import WaitOrExecute from './action-wait-handle';

class MPVMediator extends EventEmitter {
  attach(mpv: MPVElement) {
    if (!this.mpv && mpv) {
      this.mpv = mpv;
      this.mpv.addEventListener('message', e => this.handleMessage(e));
    }
  }
  getComponent(): () => Element<*> {
    return props =>
      React.createElement('embed', {
        ...props,
        ref: mpv => this.attach(mpv),
        type: 'application/x-mpvjs',
      });
  }
  handleMessage({ data: payload }: EventArgs) {
    if (payload.type === 'property_change') {
      const { name, value } = payload.data;
      this.emit(name, value);
    } else if (payload.type === 'ready') {
      this.ready = true;
    }
  }
  isReady(): boolean {
    return this.ready;
  }

  fullscreen() {
    this.mpv.webkitRequestFullscreen();
  }
  sendCommand({ name, args }) {
    args = args.map(arg => arg.toString());
    this.postData('command', [name].concat(args));
    return Observable.of({ name, args });
  }

  changeProperty(changePropertyPayload) {
    this.postData('set_property', changePropertyPayload);
    return Observable.of(changePropertyPayload);
  }

  observeProperty(name: string): Observable<*> {
    this.postData('observe_property', name);
    return Observable.fromEvent(this, name);
  }

  postData = (type: string, data: mixed) =>
    new WaitOrExecute(
      () => this.mpv.postMessage({ type, data }),
      () => this.isReady()
    ).exec();
}

export default new MPVMediator();
