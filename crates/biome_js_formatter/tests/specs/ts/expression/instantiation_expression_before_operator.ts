// `make<T> + y` reads as `make < T > +y`, so these parentheses must be kept.
x = (make<T>) + y;
x = (make<T>) - y;
x = (a + make<T>) - y;
x = (typeof make<T>) + y;
x = (<any>(make<T>)) + y;

// Other operators parse fine after the type arguments.
x = (make<T>) * y;
x = (make<T>) <= y;
x = (make<T>) === y;
x = y + make<T>;
