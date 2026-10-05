/* should generate diagnostics */
void (() => {})();
function log() {}
void log();
void 1;
void "0";
void undefined;
void { value: 1 };
void function () {};
void /* keep this comment */ log();
void // keep this line comment
    log();
void log() // trailing comment
void log()
void "use strict";
void function () {}();
const object = {}
void log();
const callback = () => {}
void log();
