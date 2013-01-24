#!/usr/bin/env node
'use strict';

var optimist = require('optimist');
var server = require('../lib/server');

var argv = optimist
  .usage('Usage: webserver.js [options]')
  .options('port', {
    alias: 'p',
    default: 8080,
    describe: 'The port to run a web server on.'
  })
  .alias('help', 'h')
  .alias('h', '?')
  .argv;

if (argv.help) {
  optimist.showHelp();
  process.exit(1);
}

process.on('uncaughtException', function(err) {
  console.error('uncaughtException', err.stack || err);
});

server(argv);
