/// <reference path="./types.d.ts" />

const stackFontSize = '14pt';
const stackLRPadding = '5px';
const buttonHeightPx = 30;
const buttonPaddingPx = 2;
const buttonWidthPx = 50;

export const menu = {
  menubar: {
    listStyleType: 'none',
    margin: 0,
    padding: 0,
    width: '100%',
    backgroundColor: 'rgb(78,78,78)'
  },
  
  link: {
    paddingLeft: '5px',
    paddingRight: '5px',
    paddingTop: '5px',
    paddingBottom: '5px',
    textDecoration: 'none',
    color: 'rgb(255,255,255)',
    fontFamily: 'Verdana,sans-serif'
  },
  
  menubarLink: {
    display: 'block'    
  },
  
  linkContainer: {
    display: 'flex',
    ':hover': {
      backgroundColor: 'rgb(0,0,0)'
    }
  },
  
  menubarLinkContainer: {
    
  },
  
  submenuItem: {
  },

  menubarSubmenuItem: {
    float: 'left',
    display: 'block'
  },
  
  submenu: {
    position: 'fixed',
    listStyleType: 'none',
    margin: 0,
    padding: 0
  },
  
  menubarSubmenu: {
    position: 'fixed',
    listStyleType: 'none',
    margin: 0,
    padding: 0,
    backgroundColor: 'rgb(100,100,100)',
    minWidth: '150px'
  },
  
  menuItemLeft: {
    width: '15px',
    textAlign: 'center',
    color: 'white',
    paddingTop: '5px',
    paddingLeft: '7px'
  }
};

export const error = {
  background: '#ffdddd',
  padding: '5px'
};

export const app = {
  container: {
    display: 'flex',
    flexDirection: 'column',
    height: '100%',
    width: '100%'
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
    overflowY: 'scroll',
    display: 'flex',
    flexDirection: 'column'
  },
  
  beforeSpacer: {
    flexGrow: 1
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