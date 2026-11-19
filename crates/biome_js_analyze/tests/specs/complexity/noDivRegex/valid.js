/* should not generate diagnostics */
var f = function() { return /foo/ig.test('bar'); };

var f = function() { return /\\=foo/; };
