/* should generate diagnostics */

p.finally(() => {
    {
        return 1;
    }
});

p.finally(() => {
    if (condition) {
        return 1;
    }
});

p.finally(function () {
    if (condition) return 1;
});

p.finally(function () {
    if (condition) {
        cleanup();
    } else {
        return 1;
    }
});

p.finally(() => {
    for (const item of items) {
        if (item) return item;
    }
});

p.finally(function () {
    while (condition) {
        return 1;
    }
});

p.finally(() => {
    switch (value) {
        case 1:
            return 1;
    }
});

p.finally(function () {
    try {
        return 1;
    } catch (error) {
        cleanup(error);
    }
});

p.finally(() => {
    try {
        cleanup();
    } catch (error) {
        return 1;
    }
});

p.finally(() => {
    try {
        cleanup();
    } finally {
        return 1;
    }
});

p.finally(() => {
    label: {
        return;
    }
});

p.finally(() => {
    function nested() {
        return 1;
    }
    if (condition) {
        return 2;
    }
});
