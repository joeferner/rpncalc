'use strict';

var fs = require('fs');
var path = require('path');
var async = require('async');
var Rpncalc = require('./rpncalc');

module.exports = function(app) {
  app.get('/help', getHelp);
  app.get('/app/rpncalcFunctions.js', getRpnCalcFunctions);
  app.get('/rpncalc', getRpncalc);
  app.get('/graph', getGraph);
  app.post('/rpncalc/push', postRpncalcPush);
  app.post('/rpncalc/execute', postRpncalcExecute);

  function getRpncalc(req, res, next) {
    var rpncalc = JSON.parse(JSON.stringify(app.rpncalc));
    for (var i = 0; i < app.rpncalc.stack.length; i++) {
      rpncalc.stack[i] = app.rpncalc.stack[i].toString(app.rpncalc.numBase);
    }
    //console.log(JSON.stringify(rpncalc));
    return res.json(rpncalc);
  }

  function postRpncalcExecute(req, res, next) {
    try {
      var commandName = req.body.commandName;
      //console.log('postRpncalcExecute:', commandName);
      app.rpncalc[commandName]();
      return res.json({});
    } catch (err) {
      return res.status(400).json({ message: err.message });
    }
  }

  function postRpncalcPush(req, res, next) {
    try {
      var value = req.body.value;
      //console.log('postRpncalcPush:', value);
      app.rpncalc.push(value);
      return res.json({});
    } catch (err) {
      return res.status(400).json({ message: err.message });
    }
  }

  function getRpnCalcFunctions(req, res, next) {
    return getCommands(function(err, commands) {
      if (err) {
        return next(err);
      }
      res.setHeader('Content-Type', 'text/javascript');
      return res.end('define({ commands: ' + JSON.stringify(commands) + ' });');
    });
  }

  function getGraph(req, res, next) {
    return res.render('graph.ejs', {
      eq1: req.query.eq1
    });
  }

  function getHelp(req, res, next) {
    return async.auto({
      commands: getGroupedCommands
    }, function(err, results) {
      if (err) {
        return next(err);
      }
      return res.render('help.ejs', {
        commands: results.commands
      });
    });
  }

  function getCommands(callback) {
    var commands = [];
    Object.keys(Rpncalc.prototype).forEach(function(name) {
      var fn = Rpncalc.prototype[name];
      if (typeof(fn) == 'function' && fn.description) {
        commands.push({
          name: fn.altName || name,
          fullName: name,
          description: fn.description,
          category: fn.category || 'General'
        });
      }
    });
    return callback(null, commands);
  }

  function getGroupedCommands(callback) {
    return getCommands(function(err, commands) {
      if (err) {
        return callback(err);
      }

      var groups = groupBy(commands, function(item) { return item.category; });
      Object.keys(groups).forEach(function(groupName) {
        groups[groupName] = groups[groupName].sort(function(a, b) {
          if (a.name < b.name) {
            return -1;
          }
          if (a.name > b.name) {
            return 1;
          }
          return 0;
        });
      });
      return callback(null, groups);
    });
  }

  function groupBy(arr, fn) {
    var groups = {};
    arr.forEach(function(item) {
      var groupName = fn(item);
      groups[groupName] = groups[groupName] || [];
      groups[groupName].push(item);
    });
    return groups;
  }
};
