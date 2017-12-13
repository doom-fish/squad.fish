//@flow
const WAIT_TIME = 1000;
const TIMEOUT = 5000;
type ActionType = () => void;
type PredicateType = () => boolean;

export default class ActionWaitHandle {
  waited: number = 0;
  action: ActionType;
  predicate: PredicateType = () => true;
  constructor(action: ActionType, predicate: PredicateType) {
    this.action = action;
    this.predicate = predicate;
  }
  exec() {
    if (this.predicate()) {
      this.action();
    } else {
      if (this.waited > TIMEOUT) {
        throw new Error(
          `Error executing action in MPV Mediator, mpv not ready after waiting ${TIMEOUT}ms`
        );
      } else {
        this.waited += WAIT_TIME;
      }
      setTimeout(() => this.exec(), WAIT_TIME);
    }
  }
}
