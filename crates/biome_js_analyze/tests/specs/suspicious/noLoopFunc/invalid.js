/* should generate diagnostics */
for (var i = 0; i < 10; i++) {
    handlers.push(() => i);
}

var count = 0;
while (count < 5) {
    function next() {
        return count;
    }

    queue.push(next);
    count++;
}

let shared = 0;
for (let i = 0; i < 10; i++) {
    queue.push(function () {
        return shared;
    });
}
shared = 100;

for (var i = 0; i < 5; i++) {
    queue.push((() => {
        return () => i;
    })());
}

for (var i = 0; i < 5; i++) {
    (async () => i)();
}
