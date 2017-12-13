import { lookForChromecasts } from 'redux/modules/casts/actions';
import { listenForStream } from 'redux/modules/stream/actions';
export default async store => {
  await store.dispatch(lookForChromecasts());
  await store.dispatch(listenForStream());
};
