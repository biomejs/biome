/* should not generate diagnostics */
const fs = require('fs');
module.exports = function readFile(filename, callback) {
	fs.readFile(filename, callback);
};
