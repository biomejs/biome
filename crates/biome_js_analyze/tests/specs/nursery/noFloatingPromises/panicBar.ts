import type {Foo} from './panicFoo';

let instance: Foo;

// This call expression makes the `noFloatingPromises` rule work.
instance.doSomething();
