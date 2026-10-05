/* should not generate diagnostics */
void 0;
void (0);
void 0x0;
function value() { return 1; }
void value();
void Promise.resolve();
const promise = Promise.resolve(1);
void promise;
void new Promise(resolve => resolve());
const thenable = { then(callback) { callback(); } };
void thenable;
!value();
+value();
