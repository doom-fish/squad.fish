import { Component } from 'react';
export default renderFunc => {
  return class PureComponent extends Component {
    shouldComponentUpdate(nextProps) {
      const nextPropKeys = Object.keys(nextProps);
      const noHandlerProps = nextPropKeys.filter(key => !key.startsWith('on'));
      const noChanges = noHandlerProps.reduce((changes, propsKey) => {
        return changes && this.props[propsKey] === nextProps[propsKey];
      }, true);
      return !noChanges;
    }
    render() {
      return renderFunc(this.props);
    }
  };
};
