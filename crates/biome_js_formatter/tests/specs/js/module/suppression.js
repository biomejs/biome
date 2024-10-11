// biome-ignore format: the following if should print inline
if(true) statement();

/** biome-ignore format: the following if should print inline */
if (true) statement();

/**
 * biome-ignore format: the following if should print inline
 */
if (true) statement();

const   expr   =
// biome-ignore format: the array should not be formatted
[
    (2*n)/(r-l), 0,            (r+l)/(r-l),  0,
    0,           (2*n)/(t-b),  (t+b)/(t-b),  0,
    0,           0,           -(f+n)/(f-n), -(2*f*n)/(f-n),
    0,           0,           -1,            0,
];

const    expr2    =    {
    key:
        // biome-ignore format: only skip formatting the value
        'single quoted string'
}

let a =
    // biome-ignore format: test
function () {}
