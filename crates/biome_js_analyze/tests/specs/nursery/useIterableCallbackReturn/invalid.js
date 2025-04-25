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
[].filter(() => {
    return;
});
[].filter(function() {
    return;
});
[].filter(() => {});
[].filter(function() {});
[].find(() => {
    return;
});
[].find(function() {
    return;
});
[].find(() => {});
[].find(function() {});
[].findIndex(() => {
    return;
});
[].findIndex(function() {
    return;
});
[].findIndex(() => {});
[].findIndex(function() {});
[].findLast(() => {
    return;
});
[].findLast(function() {
    return;
});
[].findLast(() => {});
[].findLast(function() {});
[].findLastIndex(() => {
    return;
});
[].findLastIndex(function() {
    return;
});
[].findLastIndex(() => {});
[].findLastIndex(function() {});
[].some(() => {
    return;
});
[].some(function() {
    return;
});
[].some(() => {});
[].some(function() {});
[].flatMap(() => {
    return;
});
[].flatMap(function() {
    return;
});
[].flatMap(() => {});
[].flatMap(function() {});
[].map(() => {
    return;
});
[].map(function() {
    return;
});
[].map(() => {});
[].map(function() {});
[].reduce((a, b) => {
    return;
});
[].reduce(function(a, b) {
    return;
});
[].reduce((a, b) => {});
[].reduce(function(a, b) {});
[].reduceRight((a, b) => {
    return;
});
[].reduceRight(function(a, b) {
    return;
});
[].reduceRight((a, b) => {});
[].reduceRight(function(a, b) {});
[].sort((a, b) => {
    return;
});
[].sort(function(a, b) {
    return;
});
[].sort((a, b) => {});
[].sort(function(a, b) {});
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
