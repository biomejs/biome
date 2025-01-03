export function f(x) {
    if(x) {
        // assign 'y'
        var /* @type number */ y /*: number */ = 2*x;
        // assign 'y' to 'x'
        x = y;
    }
    return x;
}