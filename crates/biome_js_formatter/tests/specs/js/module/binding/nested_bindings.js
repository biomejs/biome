({normal, long, includes, multiple, bindings}) => { hello(); }
({normal, long, includes, multiple, bindings, enough, to, pass, the, lineLength}) => { hello(); }

({normal: something, n: {yes: what, layout: {e}}}) => { bar(); };

({n: {l: {e}}}) => { bar(); };

const obj = {

    flat(id, {  title }) {
        return id + title;
      },

    nested(id, { blog: { title } }) {
      return id + title;
    },

    twice(id, {blog: { middle: {title}}}) {
      return id + title;
    }
  };

  class A {
    flat(id, {title}) {
        return id + title;
    }
    func(id, { blog: { title } }) {
      return id + title;
    }

    twice(id, {blog: { middle: {title}}}) {
      return id + title;
    }
  };
  
  try {
    foo();
} catch({error}) {
    nothing();
} 
  
try {
    foo();
} catch({error: {nestedOnce}}) {
    nothing();
} 

try {
    foo();
} catch({error: {nested: {twice}}}) {
    nothing();
}