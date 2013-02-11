'use strict';

$(function() {
  var keys = document.require('../../lib/keys');
  var stackElem = document.getElementById('stack');
  var errorElem = document.getElementById('error');
  var stackTemplate = new EJS({ element: 'stackTemplate' });

  var inputElem = document.getElementById('stackInput');
  inputElem.onblur = function() {
    setTimeout(function() {
      inputElem.focus();
    }, 100);
  };

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
    var jqxhr = $.get(
      '/rpncalc',
      function(rpncalc) {
        updateStatusBar(rpncalc);
        updateStack(rpncalc);
        onWindowResize();
      })
      .fail(function() {
        var json = JSON.parse(jqxhr.responseText);
        console.log('stack update failed:', json.message);
        displayError(new Error(json.message));
      });
  }

  function updateStack(rpncalc) {
    var stackInputValue = '';
    if (inputElem) {
      stackInputValue = $(inputElem).val();
    }
    stackElem.innerHTML = stackTemplate.render({
      rpncalc: rpncalc,
      stackItemsToDisplay: 50,
      stackInputValue: stackInputValue
    });
    $('.stackItem').click(onStackItemClick);
    inputElem.focus();
    scrollStackToBottom();
  }

  function onStackItemClick() {
    var m = $(this).attr('id').match(/^stackItem_(.*)$/);
    if (m) {
      var id = parseInt(m[1]);
      var val = $('#stackItem_' + id + ' .stackItemValue').html();
      if (val && val.length > 0) {
        inputElem.value = val;
      }
    }
  }

  function scrollStackToBottom() {
    var stackItemsContainerElem = document.getElementById('stackItemsContainer');
    stackItemsContainerElem.scrollTop = stackItemsContainerElem.scrollHeight;
  }

  function updateStatusBar(rpncalc) {
    $('#angleMode').html(angleModeToString(rpncalc.angleMode));
    $('#numBase').html(numBaseToString(rpncalc.numBase));
    return 0;

    function angleModeToString(angleMode) {
      switch (angleMode) {
      case 'rad':
        return 'Radians';
      case 'deg':
        return 'Degrees';
      default:
        return 'Unknown: ' + angleMode
      }
    }

    function numBaseToString(numBase) {
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
    }
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

  function push(val, callback) {
    callback = callback || function() {};
    var jqxhr = $.post(
      '/rpncalc/push',
      {
        value: val
      },
      function() {
        $(inputElem).val('');
        return callback();
      })
      .fail(function() {
        var json = JSON.parse(jqxhr.responseText);
        var err = new Error(json.message);
        displayError(err);
        return callback(err);
      });
  }

  function pushInput(callback) {
    var val = getStackInputValue();
    if (val.length == 0) {
      return callback();
    }
    push(val, callback);
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
        switch (event.which) {
        case keys.PLUS:
          execute('plus');
          break;
        case keys.MINUS:
          execute('subtract');
          break;
        case keys.ASTERISK:
          execute('multiply');
          break;
        case keys.FORWARD_SLASH:
          execute('divide');
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
          pushInput(function() {
            update();
          });
          break;
        }
        return;
      }

      switch (event.which) {
      case keys.BACKSPACE:
        if (getStackInputValue().length > 0) {
          // do nothing
        } else {
          execute('drop');
        }
        break;

      case keys.ENTER:
        pushInput(function() {
          update();
        });
        break;

      default:
        console.log('onKeyDown', event.which);
      }
    } catch (e) {
      displayError(e);
    }
  }

  function execute(fn) {
    pushInput(function() {
      push(fn, function() {
        update();
      });
    });
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
        pushInput(function() {
          update();
        });
        break;

      case 'drop':
        execute('drop');
        break;

      case 'swap':
        execute('swap');
        break;

      case '+/-':
        if (val.length > 0) {
          if (inputElem.value[0] == '-') {
            inputElem.value = inputElem.value.substr(1);
          } else {
            inputElem.value = '-' + inputElem.value;
          }
        } else {
          execute('neg');
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
        pushInput(function() {
          inputElem.value = Math.PI;
          pushInput(function() {
            update();
          });
        });
        break;

      default:
        execute(key);
        break;
      }
      inputElem.focus();
    } catch (e) {
      displayError(e);
    }
  }
});

