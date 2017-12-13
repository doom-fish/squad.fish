import React, { Component, Element } from 'react';

import PropTypes from 'prop-types';
import { Observable, Subject } from 'rxjs';

const prioritize = s$ => {
  var first = new Subject();
  var second = s$.do(x => first.next(x)).share();

  return [
    Observable.using(() => second.subscribe(() => {}), () => first),
    second,
  ];
};

const toObservable = (subject: mixed, event: string) =>
  Observable.fromEvent(subject, event).do(x => x.preventDefault());

type DragObservablesType = {
  dragStart: Observable<*>,
  dragMove: Observable<*>,
  dragEnd: Observable<*>,
};

export default (ComponentToBeDragged: any): any =>
  class DraggableComponent extends Component<*, *, *> {
    target: Element<*>;
    static propTypes = {
      onDrag: PropTypes.func,
      onDragStart: PropTypes.func,
      onDragEnd: PropTypes.func,
      className: PropTypes.string,
      style: PropTypes.object,
    };
    componentDidMount() {
      const { dragStart, dragMove, dragEnd } = this.setupDragObservables();
      const { onDrag, onDragStart, onDragEnd } = this.props;
      if (onDrag) {
        dragMove.subscribe(event => onDrag(event));
      }
      if (onDragStart) {
        dragStart.subscribe(event => onDragStart(event));
      }
      if (onDragEnd) {
        dragEnd.subscribe(event => onDragEnd(event));
      }
    }
    setupDragObservables(): DragObservablesType {
      const mouseup = toObservable(document, 'mouseup');
      const mousemove = toObservable(document, 'mousemove');
      const mousedown = toObservable(this.target, 'mousedown');

      const [dragStartMouseDown, dragMoveMouseDown] = prioritize(mousedown);
      const dragStart = dragStartMouseDown.flatMap(() =>
        mousemove
          .filter(x => x.movementX !== 0 || x.movementY !== 0)
          .takeUntil(mouseup)
          .take(1)
      );
      const dragMove = dragMoveMouseDown.flatMap(() =>
        mousemove
          .filter(x => x.movementX !== 0 || x.movementY !== 0)
          .takeUntil(mouseup)
      );
      const dragEnd = dragStartMouseDown.switchMap(() => mouseup.take(1));
      return { dragStart, dragMove, dragEnd };
    }
    render(): Element<*> {
      const { className, style, ...otherProps } = this.props;

      return (
        <div
          className={className}
          style={style}
          ref={target => (this.target = target)}
        >
          <ComponentToBeDragged {...otherProps} />
        </div>
      );
    }
  };
