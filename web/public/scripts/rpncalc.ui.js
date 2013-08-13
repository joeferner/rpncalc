'use strict';

$(function() {
  var urlPrefix = 'http://localhost:9999';
  var keys = {
    BACKSPACE: 8,
    ENTER: 13,
    PLUS: 43,
    MINUS: 45,
    ASTERISK: 42,
    FORWARD_SLASH: 47
  };

  var stackElem = document.getElementById('stack');
  var errorElem = document.getElementById('error');
  var stackTemplate = new EJS({ element: 'stackTemplate' });

  var inputElem = document.getElementById('stackInput');
  inputElem.onblur = function() {
    setTimeout(function() {
      inputElem.focus();
    }, 100);
  };

  setTimeout(load, 500);
  setTimeout(onWindowResize, 100);

  $('#buttons button').click(onButtonClick);
  $('body').keypress(onKeyPress);
  $('body').keydown(onKeyDown);
  $(window).resize(onWindowResize);

  function onWindowResize() {
    scrollStackToBottom();
  }

  function load() {
    var gui = require('nw.gui');
    var fs = require('fs');
    var path = require('path');
    var settingsDir = path.resolve(getUserHome(), '.rpncalc');

    var win = gui.Window.get();
    var menubar = new gui.Menu({ type: 'menubar' });

    var fileMenu = new gui.Menu();
    fileMenu.append(new gui.MenuItem({
      label: 'Clear',
      click: function() {
        clear();
      }
    }));
    fileMenu.append(new gui.MenuItem({
      label: 'Close',
      click: function() {
        win.close();
      }
    }));
    menubar.append(new gui.MenuItem({ label: 'File', submenu: fileMenu }));

    var viewMenu = new gui.Menu();
    viewMenu.append(new gui.MenuItem({
      type: 'checkbox',
      checked: true,
      label: 'Digit Grouping',
      click: function() {
        setDigitGrouping(setDigitGroupingMenuItem.checked, function() {
          update();
        });
      }
    }));
    menubar.append(new gui.MenuItem({ label: 'View', submenu: viewMenu }));
    var setDigitGroupingMenuItem = viewMenu.items[0];

    var helpMenu = new gui.Menu();
    helpMenu.append(new gui.MenuItem({
      label: 'Help Topics',
      click: function() {
        showHelp();
      }
    }));
    menubar.append(new gui.MenuItem({ label: 'Help', submenu: helpMenu }));

    win.menu = menubar;

    win.on('close', function() {
      saveState(function() {
        win.close(true);
      });
    });

    fs.readFile(path.join(settingsDir, 'port'), 'utf-8', function(err, data) {
      urlPrefix = 'http://localhost:' + data;
      fs.readFile(path.resolve(settingsDir, 'rpncalcState.json'), function(err, data) {
        if(data) {
          data = JSON.parse(data);
          console.log(data);
        }
        update();
      });
    });
  }

  function update() {
    var jqxhr = $.get(
      urlPrefix + '/rpncalc',
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
    if(stackItemsContainerElem) {
      stackItemsContainerElem.scrollTop = stackItemsContainerElem.scrollHeight;
    }
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
      urlPrefix + '/rpncalc/push',
      {
        value: val
      },
      function(data) {
        $(inputElem).val('');
        if(data && data.fn == 'graph') {
          showGraph(data);
        }
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

  function isExponentionalNotation(v) {
    return /^([0-9\.-]+)e([0-9]*)$/.test(v);
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
        switch (event.which) {
        case keys.PLUS:
          event.preventDefault();
          execute('plus');
          break;
        case keys.MINUS:
          if (isExponentionalNotation(getStackInputValue())) {
            return;
          } else {
            event.preventDefault();
            execute('subtract');
          }
          break;
        case keys.ASTERISK:
          event.preventDefault();
          execute('multiply');
          break;
        case keys.FORWARD_SLASH:
          event.preventDefault();
          execute('divide');
          break;
        default:
          event.preventDefault();
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

  function getUserHome() {
    return process.env[(process.platform == 'win32') ? 'USERPROFILE' : 'HOME'];
  }

  function clear() {
    var jqxhr = $.post(
      urlPrefix + '/rpncalc/clear',
      function() {
        update();
        return callback();
      }).fail(function() {
        var json = JSON.parse(jqxhr.responseText);
        var err = new Error(json.message);
        displayError(err);
        return callback(err);
      });
  }

  function saveState(callback) {
    var jqxhr = $.post(
      urlPrefix + '/rpncalc/save',
      function() {
        return callback();
      }).fail(function() {
        var json = JSON.parse(jqxhr.responseText);
        var err = new Error(json.message);
        displayError(err);
        return callback(err);
      });
  }

  function setDigitGrouping(enabled, callback) {
    var jqxhr = $.post(
      urlPrefix + '/rpncalc/setDigitGrouping',
      {
        enabled: enabled
      },
      function() {
        return callback();
      }).fail(function() {
        var json = JSON.parse(jqxhr.responseText);
        var err = new Error(json.message);
        displayError(err);
        return callback(err);
      });
    return callback();
  }

  function showHelp() {
    var gui = require('nw.gui');
    gui.Window.open(urlPrefix + '/help', {
      width: 600,
      height: 500,
      toolbar: false
    });
  }

  function showGraph(graphOpts) {
    var graphWindow = gui.Window.open(urlPrefix + '/graph?eq1=' + graphOpts.equation, {
      width: 600,
      height: 500
    });

    graphWindow.on('create', function() {
      graphWindow.frame.show();
    });
  }
});

