/* should not generate diagnostics */
var AppHeader = require("app-header");
var appHeader = new AppHeader();
var appHeader = new (require("app-header"))();
var appHeader = new Require("app-header");
var appHeader = new foo.require("app-header");

function shadowed(require) {
	return new require("app-header");
}
