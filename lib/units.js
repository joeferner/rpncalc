'use strict';

var units = module.exports = [
];

/*********** Length (meters) ***********/
units.push({ alias: 'in', name: 'inches', type: 'length', standardMultiplier: 0.0254 });
units.push({ alias: 'ft', name: 'feet', type: 'length', standardMultiplier: 0.3048 });
units.push({ alias: 'yd', name: 'yards', type: 'length', standardMultiplier: 0.914399999999999 });
units.push({ alias: 'mi', name: 'miles', type: 'length', standardMultiplier: 1609.344 });
addSiUnits('m', 'meters', 'length');

function addSiUnits(aliasSuffix, nameSuffix, type) {
  units.push({ alias: 'Y' + aliasSuffix, name: 'yotta' + nameSuffix, type: type, standardMultiplier: 1e24 });
  units.push({ alias: 'Z' + aliasSuffix, name: 'zetta' + nameSuffix, type: type, standardMultiplier: 1e21 });
  units.push({ alias: 'E' + aliasSuffix, name: 'exa' + nameSuffix, type: type, standardMultiplier: 1e18 });
  units.push({ alias: 'P' + aliasSuffix, name: 'peta' + nameSuffix, type: type, standardMultiplier: 1e15 });
  units.push({ alias: 'T' + aliasSuffix, name: 'tera' + nameSuffix, type: type, standardMultiplier: 1e12 });
  units.push({ alias: 'G' + aliasSuffix, name: 'giga' + nameSuffix, type: type, standardMultiplier: 1e9 });
  units.push({ alias: 'M' + aliasSuffix, name: 'mega' + nameSuffix, type: type, standardMultiplier: 1e6 });
  units.push({ alias: 'k' + aliasSuffix, name: 'kilo' + nameSuffix, type: type, standardMultiplier: 1e3 });
  units.push({ alias: 'h' + aliasSuffix, name: 'hecto' + nameSuffix, type: type, standardMultiplier: 1e2 });
  units.push({ alias: 'da' + aliasSuffix, name: 'deca' + nameSuffix, type: type, standardMultiplier: 1e1 });
  units.push({ alias: '' + aliasSuffix, name: '' + nameSuffix, type: type, standardMultiplier: 1 });
  units.push({ alias: 'd' + aliasSuffix, name: 'deci' + nameSuffix, type: type, standardMultiplier: 1e-1 });
  units.push({ alias: 'c' + aliasSuffix, name: 'centi' + nameSuffix, type: type, standardMultiplier: 1e-2 });
  units.push({ alias: 'm' + aliasSuffix, name: 'milli' + nameSuffix, type: type, standardMultiplier: 1e-3 });
  units.push({ alias: 'u' + aliasSuffix, name: 'micro' + nameSuffix, type: type, standardMultiplier: 1e-6 });
  units.push({ alias: 'n' + aliasSuffix, name: 'nano' + nameSuffix, type: type, standardMultiplier: 1e-9 });
  units.push({ alias: 'p' + aliasSuffix, name: 'pico' + nameSuffix, type: type, standardMultiplier: 1e-12 });
  units.push({ alias: 'f' + aliasSuffix, name: 'femto' + nameSuffix, type: type, standardMultiplier: 1e-15 });
  units.push({ alias: 'a' + aliasSuffix, name: 'atto' + nameSuffix, type: type, standardMultiplier: 1e-18 });
  units.push({ alias: 'z' + aliasSuffix, name: 'zepto' + nameSuffix, type: type, standardMultiplier: 1e-21 });
  units.push({ alias: 'y' + aliasSuffix, name: 'yocto' + nameSuffix, type: type, standardMultiplier: 1e-24 });
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
