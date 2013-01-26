#!/usr/bin/env node --harmony
'use strict';

var openport = require('openport');
var appjs = require('appjs');
var path = require('path');
var server = require('../lib/server');
var optimist = require('optimist');
var fs = require('fs');

var args = optimist
  .alias('h', 'help')
  .alias('h', '?')
  .argv;

if (args.help) {
  optimist.showHelp();
  return process.exit(-1);
}

var settingsDir = path.resolve(getUserHome(), '.rpncalc');
console.log('settingsDir:', settingsDir);
var rpncalcStateFileName = path.resolve(settingsDir, 'rpncalcState.json');

function readRpnCalcState(callback) {
  return fs.mkdir(path.resolve(settingsDir), function(err) {
    if (err && err.message.indexOf('EEXIST') < 0) {
      return callback(err);
    }
    return fs.readFile(rpncalcStateFileName, 'utf8', callback);
  });
}

readRpnCalcState(function(err, rpncalcState) {
  if (err) {
    return console.error('Could not read settings', err);
  }
  return openport.find(function(err, port) {
    if (err) {
      return console.error('Could not find open port', err);
    }

    server({
      port: port
    });

    appjs.serveFilesFrom(path.resolve(__dirname, '../web/public/loading'));

    var menubar = appjs.createMenu([
      {
        label: '&File',
        submenu: [
          {
            label: '&Clear',
            action: function() {
              window.document.clearState();
            }
          },
          {
            label: 'E&xit',
            action: function() {
              window.close();
            }
          }
        ]
      }
    ]);

    var window = appjs.createWindow({
      width: 410,
      height: 500,
      icons: path.join(__dirname, 'icons'),
      url: 'http://localhost:' + port + '/'
    });

    window.on('create', function() {
      console.log("Window Created");
      window.frame.show();
      window.frame.setMenuBar(menubar);
    });

    window.on('ready', function() {
      console.log("Window Ready");
      window.require = require;
      window.process = process;
      window.module = module;
      window.rpncalcState = rpncalcState;
      window.addEventListener('keydown', function(e) {
        if (e.keyIdentifier === 'F12') {
          window.frame.openDevTools();
        }
      });
    });

    window.on('close', function() {
      console.log("Window Closed");
      var state = window.document.rpncalc;
      state = window.JSON.stringify(state, null, '  ');
      console.log('saving state');
      fs.writeFile(rpncalcStateFileName, state, function(err) {
        if (err) {
          console.error('could not save state', err);
        }
        console.log('state saved:', rpncalcStateFileName);
        process.exit(0);
      });
    });

    return 0;
  });
});

function getUserHome() {
  return process.env[(process.platform == 'win32') ? 'USERPROFILE' : 'HOME'];
}
