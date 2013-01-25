#!/usr/bin/env node --harmony
'use strict';

var openport = require('openport');
var appjs = require('appjs');
var path = require('path');
var server = require('../lib/server');
var optimist = require('optimist');

var args = optimist
  .alias('h', 'help')
  .alias('h', '?')
  .argv;

if (args.help) {
  optimist.showHelp();
  return process.exit(-1);
}

openport.find(function(err, port) {
  if (err) {
    return console.error('Could not find open port', err);
  }

  server({
    port: port
  });

  appjs.serveFilesFrom(path.resolve(__dirname, '../web/public/loading'));

  var window = appjs.createWindow({
    width: 400,
    height: 600,
    icons: path.join(__dirname, 'icons'),
    url: 'http://localhost:' + port + '/'
  });

  window.on('create', function() {
    console.log("Window Created");
    window.frame.show();
  });

  window.on('ready', function() {
    console.log("Window Ready");
    window.require = require;
    window.process = process;
    window.module = module;
    window.addEventListener('keydown', function(e) {
      if (e.keyIdentifier === 'F12') {
        window.frame.openDevTools();
      }
    });
  });

  window.on('close', function() {
    console.log("Window Closed");
    process.exit(0);
  });
});

