module.exports = function(content) {
  return "module.exports = (typeof nw == 'undefined') ? undefined : nw;";
};