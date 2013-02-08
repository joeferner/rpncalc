'use strict';

$(function() {
  var templateHelpers = createTemplateHelpers();
  var keys = document.require('../../lib/keys');
  var RpnCalc = document.require('../../');
  var rpncalc = document.rpncalc = new RpnCalc();
  var stackElem = document.getElementById('stack');
  var statusBarElem = document.getElementById('statusBar');
  var errorElem = document.getElementById('error');
  var inputElem = null;
  var statusBarTemplate = new EJS({ element: 'statusBarTemplate' });
  var stackTemplate = new EJS({ element: 'stackTemplate' });
  update();
  setTimeout(onWindowResize, 100);
  setTimeout(loadState, 100);

  $('#buttons button').click(onButtonClick);
  $('body').keypress(onKeyPress);
  $('body').keydown(onKeyDown);
  $(window).resize(onWindowResize);

  window.document.clearState = function() {
    rpncalc.clear();
    update();
  };

  function loadState() {
    var state = window.rpncalcState;
    if (state) {
      state = JSON.parse(state);
      rpncalc.loadState(state);
      update();
    }
  }

  function onWindowResize() {
    scrollStackToBottom();
  }

  function update() {
    updateStatusBar();
    updateStack();
    onWindowResize();
  }

  function updateStack() {
    var stackInputValue = '';
    if (inputElem) {
      stackInputValue = $(inputElem).val();
    }
    stackElem.innerHTML = stackTemplate.render({
      rpncalc: rpncalc,
      helpers: templateHelpers,
      stackItemsToDisplay: 50,
      stackInputValue: stackInputValue
    });
    $('.stackItem').click(onStackItemClick);
    inputElem = document.getElementById('stackInput');
    inputElem.onblur = function() {
      setTimeout(function() {
        inputElem.focus();
      }, 100);
    };
    inputElem.focus();
    scrollStackToBottom();
  }

  function onStackItemClick() {
    var m = $(this).attr('id').match(/^stackItem_(.*)$/);
    if (m) {
      var idx = parseInt(m[1]);
      var stackItem = rpncalc.stack[idx];
      if (stackItem) {
        inputElem.value = stackItem.toString(rpncalc.numBase);
      }
    }
  }

  function scrollStackToBottom() {
    var stackItemsContainerElem = document.getElementById('stackItemsContainer');
    stackItemsContainerElem.scrollTop = stackItemsContainerElem.scrollHeight;
  }

  function updateStatusBar() {
    statusBarElem.innerHTML = statusBarTemplate.render({
      rpncalc: rpncalc,
      helpers: templateHelpers
    });
  }

  function getStackInputValue() {
    if (inputElem) {
      return $(inputElem).val();
    }
    return '';
  }

  function displayError(err) {
    errorElem.innerHTML = err.message;
    console.error('Error:', err.message);
    $(errorElem).show();
  }

  function clearError() {
    errorElem.innerHTML = '';
    $(errorElem).hide();
  }

  function pushInput() {
    var val = getStackInputValue();
    if (val.length > 0) {
      rpncalc.push(val);
      $(inputElem).val('');
      update();
    }
  }

  function onKeyPress(event) {
    if (getStackInputValue()[0] == "'") {
      return;
    }

    try {
      switch (event.which) {
      case keys.PLUS:
      case keys.MINUS:
      case keys.ASTERISK:
      case keys.FORWARD_SLASH:
        event.preventDefault();
        pushInput();
        switch (event.which) {
        case keys.PLUS:
          rpncalc.plus();
          break;
        case keys.MINUS:
          rpncalc.subtract();
          break;
        case keys.ASTERISK:
          rpncalc.multiply();
          break;
        case keys.FORWARD_SLASH:
          rpncalc.divide();
          break;
        }
        update();
        break;
      default:
        console.log('onKeyPress', event.which);
      }
    } catch (e) {
      displayError(e);
    }
  }

  function onKeyDown(event) {
    clearError();

    try {
      if (getStackInputValue()[0] == "'") {
        switch (event.which) {
        case keys.ENTER:
          pushInput();
          break;
        }
        return;
      }

      switch (event.which) {
      case keys.BACKSPACE:
        if (getStackInputValue().length > 0) {
          // do nothing
        } else {
          rpncalc.drop();
          update();
        }
        break;

      case keys.ENTER:
        pushInput();
        break;

      default:
        console.log('onKeyDown', event.which);
      }
    } catch (e) {
      displayError(e);
    }
  }

  function onButtonClick() {
    var val = getStackInputValue();
    clearError();
    try {
      var key = $(this).attr('fn');
      if (!key) {
        key = $(this).html().trim().toLowerCase();
      }
      switch (key) {
      case 'enter':
        pushInput();
        break;

      case 'drop':
        rpncalc.drop();
        update();
        break;

      case 'swap':
        rpncalc.swap();
        update();
        break;

      case '+/-':
        if (val.length > 0) {
          if (inputElem.value[0] == '-') {
            inputElem.value = inputElem.value.substr(1);
          } else {
            inputElem.value = '-' + inputElem.value;
          }
        } else {
          rpncalc.neg();
          update();
        }
        break;

      case '0':
      case '1':
      case '2':
      case '3':
      case '4':
      case '5':
      case '6':
      case '7':
      case '8':
      case '9':
      case '.':
        inputElem.value += key;
        break;

      case 'pi':
        pushInput();
        rpncalc.push(Math.PI);
        update();
        break;

      default:
        if (rpncalc[key]) {
          pushInput();
          rpncalc[key]();
          update();
        } else {
          throw new Error("Unhandled key: " + key);
        }
        break;
      }
      inputElem.focus();
    } catch (e) {
      displayError(e);
    }
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
        case 8:
          return 'Octal';
        case 2:
          return 'Binary';
        default:
          return 'Base: ' + numBase
        }
      },

      stackItemToString: function(stackItem) {
        if (!stackItem) {
          return '&nbsp;';
        }
        return stackItem.toString(rpncalc.numBase);
      }
    };
  }
});

