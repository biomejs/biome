if (foo.bar && foo.bar.length) {}
if (foo.length || foo.bar()) {}
if (!!(!!foo.length)) {}
if (!(foo.length === 0)) {}
if (/** 1 **/ 0 === foo.length /** 2 **/) {}
if (0 < foo.length) {}
while (foo.length >= 1) {}
do {} while (/** 1 **/foo.length /** 2 **/);
for (let i = 0; (bar && !foo.length); i ++) {}
const isEmpty = foo.length < 1;
bar(foo.length >= 1)
bar(!foo.length || foo.length)
const bar = void !foo.length;
const isNotEmpty = Boolean(foo.length)
const isNotEmpty1 = Boolean(foo.length || bar)
const isEmpty1 = Boolean(!foo.length)
const isEmpty2 = Boolean(/** 1 **/foo.length === 0)
const isNotEmpty2 = !Boolean(foo.length === 0)
const isEmpty3 = !Boolean(!Boolean(foo.length === 0))
if (foo.size) {}
if (foo.size && bar.length) {}
// Space after keywords
function foo() {return!foo.length}
function foo() {throw!foo.length}
async function foo() {await!foo.length}
function * foo() {yield!foo.length}
function * foo() {yield*!foo.length}
delete!foo.length
typeof!foo.length
void!foo.length
a instanceof!foo.length
a in!foo.length
export default!foo.length
if(true){}else!foo.length
do!foo.length;while(true) {}
switch(foo){case!foo.length:{}}
for(const a of!foo.length);
for(const a in!foo.length);

class A {
    a() {
        if (this.length) {};
        while (!this.size || foo);
    }
}
