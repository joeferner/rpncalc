'use strict';

$(function() {
  var templateHelpers = createTemplateHelpers();
  var RpnCalc = document.require('../../');
  var rpncalc = document.rpncalc = new RpnCalc();
  var stackElem = document.getElementById('stack');
  var statusBarElem = document.getElementById('statusBar');
  var statusBarTemplate = new EJS({ url: '/templates/statusBar.ejs' });
  var stackTemplate = new EJS({ url: '/templates/stack.ejs' });
  update();

  rpncalc.on('change', update);
  rpncalc.on('error', onRpnCalcError);
  $('body').keypress(onKeyPress);
  $('body').keydown(onKeyDown);

  function onRpnCalcError(err) {
    console.log(err);
  }

  function update() {
    updateStatusBar();
    updateStack();
  }

  function updateStack() {
    console.log(rpncalc);
    stackElem.innerHTML = stackTemplate.render({
      rpncalc: rpncalc,
      helpers: templateHelpers,
      stackItemsToDisplay: 8
    });
  }

  function updateStatusBar() {
    statusBarElem.innerHTML = statusBarTemplate.render({
      rpncalc: rpncalc,
      helpers: templateHelpers
    });
  }

  function onKeyPress(event) {
    rpncalc.onKeyPress(event.which);
  }

  function onKeyDown(event) {
    rpncalc.onKeyDown(event.which);
  }

  function createTemplateHelpers() {
    return {
      angleModeToString: function(angleMode) {
        switch (angleMode) {
        case 'rad':
          return 'Radians';
        case 'deg':
          return 'Degrees';
        default:
          return 'Unknown: ' + angleMode
        }
      },

      numBaseToString: function(numBase) {
        switch (numBase) {
        case 10:
          return 'Decimal';
        case 16:
          return 'Hexadecimal';
        case 2:
          return 'Binary';
        default:
          return 'Base: ' + numBase
        }
      },

      stackItemToString: function(stackItem) {
        if(!stackItem) {
          return '&nbsp;';
        }
        return stackItem.value;
      }
    };
  }
});

