const a = (c && b) as boolean;
const a = <any>(c && b) as boolean;
const a = !(c && b) as boolean;

// typeof operator precedence
type T1 = typeof obj[number];
type T2 = (typeof obj)[number];
type T3 = keyof typeof obj[number];
type T4 = keyof (typeof obj)[number];
type T5 = (keyof typeof obj)['toString'];