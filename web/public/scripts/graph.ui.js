'use strict';

$(function() {
  var expressionEvaluator = document.require('../../lib/expressionEvaluator');
  var RpnCalc = document.require('../../lib/rpncalc');
  var StackItem = document.require('../../lib/stackItem');
  var sf = document.require('sf');

  $('#generateGraph').click(generateGraph);
  generateGraph();

  function generateGraph() {
    var jqxhr = $.get(
      '/rpncalc',
      function(rpncalcState) {
        var rpncalc = new RpnCalc();
        rpncalc.loadState(rpncalcState);
        var eq1 = $('#eq1').val();
        var opts = {
          minX: parseFloat($('#minX').val()),
          minY: parseFloat($('#minY').val()),
          maxX: parseFloat($('#maxX').val()),
          maxY: parseFloat($('#maxY').val())
        };
        graph(rpncalc, opts, [
          {
            equation: eq1
          }
        ]);
      })
      .fail(function() {
        var json = JSON.parse(jqxhr.responseText);
        console.log('generateGraph failed:', json.message);
        displayError(new Error(json.message));
      });
  }

  function graph(rpncalc, opts, equations) {
    var graphElem = document.getElementById('graph');
    opts.width = $(graphElem).width();
    opts.height = $(graphElem).height();
    opts.scaleX = opts.width / (opts.maxX - opts.minX);
    opts.scaleY = opts.height / (opts.maxY - opts.minY);
    console.log(opts);
    var html = '<g>';

    // x-axis
    html += sf('<line x1="{pt1.x}" y1="{pt1.y}" x2="{pt2.x}" y2="{pt2.y}" class="graphAxis" />', {
      pt1: toPoint({x: 0, y: opts.minY}, opts),
      pt2: toPoint({x: 0, y: opts.maxY}, opts)
    });

    // y-axis
    html += sf('<line x1="{pt1.x}" y1="{pt1.y}" x2="{pt2.x}" y2="{pt2.y}" class="graphAxis" />', {
      pt1: toPoint({x: opts.minX, y: 0}, opts),
      pt2: toPoint({x: opts.maxX, y: 0}, opts)
    });

    equations.forEach(function(equation) {
      html += graphEquation(rpncalc, opts, equation);
    });

    html += '</g>';
    graphElem.innerSVG = html;
  }

  function graphEquation(rpncalc, opts, equation) {
    var i;
    var points = [];
    var expressionTree = expressionEvaluator.parse(equation.equation);
    console.log(expressionTree);

    for (i = -10; i < opts.width + 10; i += 0.1) {
      var x = (i / opts.scaleX) + opts.minX;
      rpncalc.memory.x = new StackItem({value: x});
      expressionEvaluator.runTree(expressionTree, rpncalc);
      var y = rpncalc.popValues(1)[0];
      points.push({x: x, y: y});
    }

    var results = '';
    var polyPoints = [];
    for (i = 0; i < points.length; i++) {
      if (polyPoints.length > 0 && (points[i].y > opts.maxY || points[i].y < opts.minY)) {
        results += pointsToSvg(polyPoints);
        polyPoints = [];
      }
      polyPoints.push(points[i]);
    }
    results += pointsToSvg(polyPoints);
    return results;

    function pointsToSvg(points) {
      return sf('<polyline class="graphEquation" points="{points}" />\n', {points: points.map(function(pt) {
        pt = toPoint(pt, opts);
        return pt.x + ',' + pt.y;
      }).join(' ')});
    }
  }

  function toPoint(pt, opts) {
    var x = (pt.x - opts.minX) * opts.scaleX;
    var y = opts.height - ((pt.y - opts.minY) * opts.scaleY);
    return {x: x, y: y};
  }

  function displayError(err) {
    alert(err.message);
  }
});
