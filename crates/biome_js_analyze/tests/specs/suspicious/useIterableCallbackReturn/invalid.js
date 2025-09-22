[].forEach((a) => {
    return a.fn();
});
[].forEach(function(a) {
    return a.fn();
});
[].forEach((a) => {
    if (a) {
        return a.fn();
    }
});
[].forEach((a) => {
    if (a) {
        return;
    }
    return a.fn();
});
[].forEach((a) => {
    if (a) {
        return;
    }
    return a.fn();
});
[].forEach((a) => {
    if (a) {
        throw new Error();
    }
    return a.fn();
});
Array.from([], () => {});
Array.from([], function() {});
Array.from([], () => {
    return;
});
Array.from([], function() {
    return;
});
Array.from([], () => void null);
[].every(() => {
    return;
});
[].every(function() {
    return;
});
[].every(() => {});
[].every(function() {});
[].every(() => {
    try {
        // ok
    } finally {
        // ok
    }
});
[].every(() => {
    try {
        // ok
    } catch (e) {
        // ok
    } finally {
        // ok
    }
});
[].every(() => {
    try {
        return true;
    } catch (e) {
        // ok
    } finally {
        // ok
    }
});
[].every(() => {
    try {
        return true;
    } catch (e) {
        return true;
    }
});
[].every(() => {
    try {
        return true;
    } catch (e) {
        return true;
    } finally {}
});
[].every(() => void null);
[].filter(() => {
    return;
});
[].filter(function() {
    return;
});
[].filter(() => {});
[].filter(function() {});
[].filter(() => void null);
[].find(() => {
    return;
});
[].find(function() {
    return;
});
[].find(() => {});
[].find(function() {});
[].find(() => void null);
[].findIndex(() => {
    return;
});
[].findIndex(function() {
    return;
});
[].findIndex(() => {});
[].findIndex(function() {});
[].findIndex(() => void null);
[].findLast(() => {
    return;
});
[].findLast(function() {
    return;
});
[].findLast(() => {});
[].findLast(function() {});
[].findLast(() => void null);
[].findLastIndex(() => {
    return;
});
[].findLastIndex(function() {
    return;
});
[].findLastIndex(() => {});
[].findLastIndex(function() {});
[].findLastIndex(() => void null);
[].some(() => {
    return;
});
[].some(function() {
    return;
});
[].some(() => {});
[].some(function() {});
[].some(() => void null);
[].flatMap(() => {
    return;
});
[].flatMap(function() {
    return;
});
[].flatMap(() => {});
[].flatMap(function() {});
[].flatMap(() => void null);
[].map(() => {
    return;
});
[].map(function() {
    return;
});
[].map(() => {});
[].map(function() {});
[].map(() => void null);
[].reduce((a, b) => {
    return;
});
[].reduce(function(a, b) {
    return;
});
[].reduce((a, b) => {});
[].reduce(function(a, b) {});
[].reduce(() => void null);
[].reduceRight((a, b) => {
    return;
});
[].reduceRight(function(a, b) {
    return;
});
[].reduceRight((a, b) => {});
[].reduceRight(function(a, b) {});
[].reduceRight(() => void null);
[].sort((a, b) => {
    return;
});
[].sort(function(a, b) {
    return;
});
[].sort((a, b) => {});
[].sort(function(a, b) {});
[].sort(() => void null);
[].toSorted((a, b) => {
    return;
});
[].toSorted(function(a, b) {
    return;
});
[].toSorted((a, b) => {
    if (a > b) {
        return;
    } else if (a < b) {
        return;
    } else {
        return 1;
    }
});
[].toSorted((a, b) => {
    if (a > b) {
        return;
    } else if (a < b) {
        return;
    }
});
[].toSorted((a, b) => {
    if (a > b) {
        throw new Error();
    } else if (a < b) {
        return;
    }
});
[].toSorted(() => void null);
