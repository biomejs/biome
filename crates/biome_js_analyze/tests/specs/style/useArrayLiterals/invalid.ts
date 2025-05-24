var xs = new Array<string>();

var xs = new Array<number>(0, 1, 2);

var xs
/* A */ = /* B */
/* C */ new /* D */
/* E */ Array /* F */
/* G */ < /* H */
/* I */ number /* J */
/* K */ > /* L */
/* M */ () /* N */
;

void new Array<number>();

void new Array<number>(1, 2, 3);

void new Array<ReferenceType>(1, 2, 3);

void new Array<GenericReferenceType<T>>(1, 2, 3);

const xs = new Array<GenericReferenceType<T>>(1, 2, 3);

const xs = new Array<import("a")>(1, 2, 3);

const xs = new Array<typeof xs>(1, 2, 3);

void new Array<1 | 2 | 3 | 4>(1, 2, 3);

// it already has a type annotation, it should still trigger a diagnostic but should not change the existing type
var xs: string[] = new Array<number>();
