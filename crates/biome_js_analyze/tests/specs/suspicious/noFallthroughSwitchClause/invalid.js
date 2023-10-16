switch(bar) { case 0: a(); case 1: b() }

switch(foo) { case 0: a(); default: b() }

switch (bar) { case 0: a(); default: b(); case 1: c() }

switch(foo) { case 0: if (a) { break; } default: b() }

switch(foo) { case 0: try { throw 0; } catch (err) {} default: b() }

switch(foo) { case 0: while (a) { break; } default: b() }

switch(foo) { case 0: do { break; } while (a); default: b() }

switch(foo) { case 0: {} default: b() }

switch(foo) { case 0: a(); { /* falls through */ } default: b() }

switch(foo) { case 0: { /* falls through */ } a(); default: b() }

switch(foo) { case 0: if (a) { /* falls through */ } default: b() }

switch(foo) { case 0: { { /* falls through */ } } default: b() }

switch(foo) { case 0: { /* comment */ } default: b() }

switch(foo) { case 0: a(); /* falling through */ default: b() }

switch(foo) { case 0: a(); /* no break */ case 1: b(); }

switch(foo) { case 0: a(); /* no break */ /* todo: fix readability */ default: b() }

switch(foo) { case 0: { a(); /* no break */ /* todo: fix readability */ } default: b() }

switch (a) { case 1: ; case 2: ; case 3: }

function f () {
    switch (a) {
        case 0:
            try { return foo(); } catch {} finally {}
        case 1:
            f();
    }
}

switch (foo) { case 0: {} case 1: case 2: f(); }
