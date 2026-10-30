/* should generate diagnostics */
Promise.all([(await promise as Foo).bar]);
Promise.all([(await promise)!]);
Promise.all([await promise as Foo]);
Promise.all([await promise satisfies Foo]);
Promise.all([<Foo>await promise]);
Promise.all([await promise!]);
Promise.all([(await promise)<Foo>()]);
Promise.all<Foo>([await promise]);
