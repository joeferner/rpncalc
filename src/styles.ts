/// <reference path="./types.d.ts" />

const stackFontSize = '14pt';
const stackLRPadding = '5px';
const buttonHeightPx = 30;
const buttonPaddingPx = 2;
const buttonWidthPx = 50;

export const error = {
  background: '#ffdddd',
  padding: '5px'
};

export const app = {
  container: {
    display: 'flex',
    flexDirection: 'column',
    height: '100%',
    width: ((buttonWidthPx + 2 * buttonPaddingPx) * 8) + 'px'
  }
};

export const modeInfo = {
  container: {
    flex: '0 0 auto',
    display: 'flex',
    justifyContent: 'center'
  },

  angleMode: {
    flex: '0 0 auto'
  }
}

export const stack = {
  container: {
    flex: '1 1 auto',
    fontSize: stackFontSize,
    overflowY: 'scroll'
  },
  
  stackList: {
    listStyleType: 'none',
    paddingLeft: 0,
    margin: 0
  },
  
  listItem: {
    display: 'flex',
    flexDirection: 'row',
    paddingLeft: stackLRPadding,
    paddingRight: stackLRPadding,
    borderBottom: '1px solid rgb(190, 190, 190)'
  },
  
  index: {
    paddingBottom: '2px',
    paddingTop: '2px',
    paddingRight: '20px'
  },
  
  value: {
    paddingBottom: '2px',
    paddingTop: '2px',
    flexGrow: 1,
    textAlign: 'right'
  },
  
  input: {
    width: '100%',
    border: 0,
    fontSize: stackFontSize,
    textAlign: 'left',
    paddingLeft: stackLRPadding,
    paddingRight: stackLRPadding,
    outline: 'none',
    marginTop: '5px'
  }
};

export const buttons = {
  container: {
    flex: '0 0 ' + (((buttonHeightPx + 2 * buttonPaddingPx) * 4) + 4) + 'px'
  },
  
  button: {
    height: buttonHeightPx + 'px',
    padding: buttonPaddingPx + 'px ' + buttonPaddingPx + 'px ' + buttonPaddingPx + 'px ' + buttonPaddingPx + 'px ',
    width: buttonWidthPx + 'px',
    background: 'none',
    border: '1px solid #aaa',
    borderRadius: '2px'
  }
};
