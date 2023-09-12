fooooooooooooooooooooooooooooooooooooooooooooooooooo(action => next =>
    dispatch(action),
);

foo(
  ({
    a,

    b
  }) => {}
);

foo(
  ({
    a,
    b

  }) => {}
);

foo(
  ({
    a,
    b
  }) => {}
);

foo(
  a,
  ({
    a,

    b
  }) => {}
)

foo(
  ({
    a,

    b
  }) => a
);

foo(
  ({
    a,
    b
  }) => a
);

foo(
  ({
    a,
    b

  }) => a
);

foo(
  ({
    a: {
      a,

      b
    }
  }) => {}
);

foo(
  ({
    a: {
      b: {
        c,

        d
      }
    }
  }) => {}
);

foo(
  ({
    a: {
      b: {
        c: {
          d,

          e
        }
      }
    }
  }) => {}
);

foo(
  ({
    a: {
      a,

      b
    }
  }) => a
);

foo(
  ({
    a: {
      b: {
        c,

        d
      }
    }
  }) => a
);

foo(
  ({
    a: {
      b: {
        c: {
          d,

          e
        }
      }
    }
  }) => a
);

foo(
  ([
    {
      a: {
        b: {
          c: {
            d,

            e
          }
        }
      }
    }
  ]) => {}
);

foo(
  ([
    ...{
      a: {
        b: {
          c: {
            d,

            e
          }
        }
      }
    }
  ]) => {}
);

foo(
  (
    n = {
      a: {
        b: {
          c: {
            d,

            e
          }
        }
      }
    }
  ) => {}
);

foo(
  ({
    x: [
      {
        a,

        b
      }
    ]
  }) => {}
);

foo(
  (
    a = [
      {
        a,

        b
      }
    ]
  ) => a
);

foo(
  ([
    [
      {
        a,

        b
      }
    ]
  ]) => {}
);

foo(
  ([
    [
      [
        [
          {
            a,
            b: {
              c,
              d: {
                e,

                f
              }
            }
          }
        ]
      ]
    ]
  ]) => {}
);

foo(
  (
    ...{
      a,

      b
    }
  ) => {}
);

foo(
  (
    ...[
      {
        a,

        b
      }
    ]
  ) => {}
);

foo(
  ([
    ...[
      {
        a,

        b
      }
    ]
  ]) => {}
);

foo(
  (
    a = [{
      a,

      b
    }]
  ) => {}
);

foo(
  (
    a = (({
      a,

      b
    }) => {})()
  ) => {}
);

foo(
  (
    a = f({
      a,

      b
    })
  ) => {}
);

foo(
  (
    a = ({
      a,

      b
    }) => {}
  ) => {}
);

foo(
  (
    a = 1 +
      f({
        a,

        b
      })
  ) => {}
);
