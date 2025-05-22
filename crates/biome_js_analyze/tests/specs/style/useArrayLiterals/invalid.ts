var xs = new Array<string>();

var xs = new Array<number>(0, 1, 2);

var xs /* A */ = /* B */ new /* C */ Array /* D */ < /* E */ number /* F */ > /* G */ ( /* H */ 1 /* I */ , /* J */ 2 /* K */ , /* L */) /* M */;

void new Array<number>();

void new Array<number>(1, 2, 3);

void new Array<1 | 2 | 3 | 4>(1, 2, 3);

// it already has a type annotation, it should still trigger a diagnostic but should not change the existing type
var xs: string[] = new Array<number>();
