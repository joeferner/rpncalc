'use strict';

var units = module.exports = [
];

/*********** Length (meters) ***********/
units.push({ alias: 'in', name: 'inches', type: 'length', typeName: 'inches', standardMultiplier: 0.0254 });
units.push({ alias: 'ft', name: 'feet', type: 'length', typeName: 'feet', standardMultiplier: 0.3048 });
units.push({ alias: 'yd', name: 'yards', type: 'length', typeName: 'yards', standardMultiplier: 0.914399999999999 });
units.push({ alias: 'mi', name: 'miles', type: 'length', typeName: 'miles', standardMultiplier: 1609.344 });
addSiUnits('m', 'meters', 'length');
addSiUnits('', '', 'general');

function addSiUnits(aliasSuffix, nameSuffix, type) {
  units.push({ alias: 'Y' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'Y', name: 'yotta' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e24 });
  units.push({ alias: 'Z' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'Z', name: 'zetta' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e21 });
  units.push({ alias: 'E' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'E', name: 'exa' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e18 });
  units.push({ alias: 'P' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'P', name: 'peta' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e15 });
  units.push({ alias: 'T' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'T', name: 'tera' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e12 });
  units.push({ alias: 'G' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'G', name: 'giga' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e9 });
  units.push({ alias: 'M' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'M', name: 'mega' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e6 });
  units.push({ alias: 'k' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'k', name: 'kilo' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e3 });
  units.push({ alias: 'h' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'h', name: 'hecto' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e2 });
  units.push({ alias: 'da' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'da', name: 'deca' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e1 });
  units.push({ alias: '' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: '', name: '' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1 });
  units.push({ alias: 'd' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'd', name: 'deci' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-1 });
  units.push({ alias: 'c' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'c', name: 'centi' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-2 });
  units.push({ alias: 'm' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'm', name: 'milli' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-3 });
  units.push({ alias: 'u' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'u', name: 'micro' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-6 });
  units.push({ alias: 'n' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'n', name: 'nano' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-9 });
  units.push({ alias: 'p' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'p', name: 'pico' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-12 });
  units.push({ alias: 'f' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'f', name: 'femto' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-15 });
  units.push({ alias: 'a' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'a', name: 'atto' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-18 });
  units.push({ alias: 'z' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'z', name: 'zepto' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-21 });
  units.push({ alias: 'y' + aliasSuffix, aliasSuffix: aliasSuffix, unitPrefix: 'y', name: 'yocto' + nameSuffix, type: type, typeName: nameSuffix, standardMultiplier: 1e-24 });
}

units.validateUnits = function(unitStr) {
  var u = units.getUnits(unitStr);
  if (!u) {
    throw new Error("Invalid units '" + unitStr + "'");
  }
};

units.getUnits = function(unitStr) {
  for (var i = 0; i < units.length; i++) {
    if (units[i].alias == unitStr) {
      return units[i];
    }
  }
  return undefined;
};

units.convert = function(value, currentUnitsStr, newUnitsStr) {
  if (currentUnitsStr == newUnitsStr) {
    return value;
  }

  var currentUnits = units.getUnits(currentUnitsStr);
  if (!currentUnits) {
    throw new Error("Could not find units '" + currentUnitsStr + "'");
  }

  var newUnits = units.getUnits(newUnitsStr);
  if (!newUnits) {
    throw new Error("Could not find units '" + newUnitsStr + "'");
  }

  if (currentUnits.type != newUnits.type) {
    throw new Error("Inconsistent unit types '" + currentUnits.type + "' and '" + newUnits.type + "'");
  }

  console.log('convert', value, currentUnits, newUnits);

  return (value * currentUnits.standardMultiplier) / newUnits.standardMultiplier;
};

units.getSIUnits = function(value) {
  value = Math.abs(value);
  var possibleUnits = [];
  for (var i = 0; i < units.length; i++) {
    var u = units[i];
    if(u.typeName == '') {
      var adjValue = value / u.standardMultiplier;
      if(adjValue < 1000 && adjValue >= 1) {
        possibleUnits.push(u);
      }
    }
  }
  if(possibleUnits.length == 1) {
    return possibleUnits[0];
  }

  possibleUnits = possibleUnits.filter(function(u) {
    var adjValue = value / u.standardMultiplier;
    if(adjValue < 10 && adjValue >= 1) {
      return true;
    }
    return false;
  });
  if(possibleUnits.length == 1) {
    return possibleUnits[0];
  }

  return null;
};