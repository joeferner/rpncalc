'use strict';

$(function() {
  var templateHelpers = createTemplateHelpers();
  var keys = document.require('../../lib/keys');
  var RpnCalc = document.require('../../');
  var rpncalc = document.rpncalc = new RpnCalc();
  var stackElem = document.getElementById('stack');
  var statusBarElem = document.getElementById('statusBar');
  var inputElem = null;
  var currentError = null;
  var statusBarTemplate = new EJS({ element: 'statusBarTemplate' });
  var stackTemplate = new EJS({ element: 'stackTemplate' });
  update();
  setTimeout(onWindowResize, 100);

  $('#buttons button').click(onButtonClick);
  $('body').keypress(onKeyPress);
  $('body').keydown(onKeyDown);
  $(window).resize(onWindowResize);

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
      stackInputValue: stackInputValue,
      currentError: currentError
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
    var idx = parseInt(m[1]);
    var stackItem = rpncalc.stack[idx];
    if (stackItem) {
      inputElem.value = stackItem.toString(rpncalc.numBase);
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
    currentError = err;
    console.error('Error:', err.message);
    update();
  }

  function clearError() {
    if (currentError) {
      currentError = null;
      update();
    }
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

      case 'x':
      case '/':
      case '+':
      case '-':
      case 'pi':
      case '1/x':
      case 'sqrt':
      case 'x^2':
      case 'y^x':
      case 'sin':
      case 'cos':
      case 'tan':
      case 'asin':
      case 'acos':
      case 'atan':
      case 'nroot':
      case 'log':
      case 'ln':
        pushInput();
        switch (key) {
        case 'x':
          rpncalc.multiply();
          break;
        case '/':
          rpncalc.divide();
          break;
        case '+':
          rpncalc.plus();
          break;
        case '-':
          rpncalc.subtract();
          break;
        case '1/x':
          rpncalc.inv();
          break;
        case 'sqrt':
          rpncalc.sqrt();
          break;
        case 'x^2':
          rpncalc.pow2();
          break;
        case 'y^x':
          rpncalc.pow();
          break;
        case 'sin':
          rpncalc.sin();
          break;
        case 'cos':
          rpncalc.cos();
          break;
        case 'tan':
          rpncalc.tan();
          break;
        case 'asin':
          rpncalc.asin();
          break;
        case 'acos':
          rpncalc.acos();
          break;
        case 'atan':
          rpncalc.atan();
          break;
        case 'nroot':
          rpncalc.nroot();
          break;
        case 'log':
          rpncalc.log();
          break;
        case 'ln':
          rpncalc.ln();
          break;
        case 'pi':
          rpncalc.push(Math.PI);
          break;
        }
        update();
        break;

      default:
        throw new Error("Unhandled key: " + key);
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

