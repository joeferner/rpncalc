/// <reference path="./types.d.ts" />

const stackFontSize = '14pt';
const stackLRPadding = '5px';
const buttonHeightPx = 30;
const buttonPaddingPx = 2;

export const error = {
  background: '#ffdddd',
  padding: '5px'
};

export const app = {
  container: {
    display: 'flex',
    flexDirection: 'column',
    height: '100%'
  }
};

export const stack = {
  container: {
    flex: '1 1 auto',
    fontSize: stackFontSize,
    overflowY: 'scroll'
  },
  
  stackListStyle: {
    listStyleType: 'none',
    paddingLeft: 0
  },
  
  listItemStyle: {
    clear: 'both',
    paddingLeft: stackLRPadding,
    paddingRight: stackLRPadding
  },
  
  indexStyle: {
    float: 'left'
  },
  
  valueStyle: {
    float: 'right'
  },
  
  inputStyle: {
    width: '100%',
    border: 0,
    fontSize: stackFontSize,
    textAlign: 'left',
    paddingLeft: stackLRPadding,
    paddingRight: stackLRPadding,
    outline: 'none'
  }
};

export const buttons = {
  container: {
    flex: '0 0 ' + (((buttonHeightPx + 2 * buttonPaddingPx) * 4) + 4) + 'px'
  },
  
  button: {
    height: buttonHeightPx + 'px',
    padding: buttonPaddingPx + 'px ' + buttonPaddingPx + 'px ' + buttonPaddingPx + 'px ' + buttonPaddingPx + 'px ',
    width: '50px',
    background: 'none',
    border: '1px solid #aaa',
    borderRadius: '2px'
  }
};
