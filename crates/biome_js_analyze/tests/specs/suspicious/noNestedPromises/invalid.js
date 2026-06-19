/* should generate diagnostics */

doThing().then(function() { a.then() })

doThing().then(function() { b.catch() })

doThing().then(function() { return a.then() })

doThing().then(function() { return b.catch() })

doThing().then(() => { a.then() })

doThing().then(() => { b.catch() })

doThing().then(() => a.then())

doThing().then(() => b.catch())

// references vars in closure but doesn't use them
doThing()
  .then(a => getB(a)
    .then(b => getC(b))
  )

doThing()
  .then(a => getB(a)
    .then(b => getC(a, b)
      .then(c => getD(a, c))
    )
  )