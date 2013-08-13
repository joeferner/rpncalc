#!/usr/bin/env node --harmony
'use strict';

var openport = require('openport');
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
    return fs.exists(rpncalcStateFileName, function(exists) {
      if (exists) {
        return fs.readFile(rpncalcStateFileName, 'utf8', function(err, data) {
          if (err) {
            return callback(err);
          }
          try {
            return callback(null, JSON.parse(data));
          } catch (e) {
            return callback(e);
          }
        });
      } else {
        return callback(null, {});
      }
    });
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

    fs.writeFileSync(path.join(settingsDir, 'port'), port);

    var serverInstance = server({
      port: port,
      rpncalcState: rpncalcState
    });

    return 0;
  });
});

function getUserHome() {
  return process.env[(process.platform == 'win32') ? 'USERPROFILE' : 'HOME'];
}
