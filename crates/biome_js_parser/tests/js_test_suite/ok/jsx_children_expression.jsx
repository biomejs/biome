let x;
let a;
let b;
let key;
let f = () => {};
<div>
  {1}
  {9007199254740991n}
  {""}
  {true}
  {null}
  {undefined}
  {/a/}
  {[]}
  {x => console.log(x)}
  {x = 1}
  {await x}
  {1 + 1}
  {f()}
  {a[b]}
  {a?1:2}
  {function f() {}}
  {function () {}}
  {a}
  {import("a.js")}
  {key in a}
  {a instanceof Object}
  {a && b}
  {new f()}
  {{}}
  {(a)}
  {a++}
  {++a}
  {a,b}
  {a.b}
  {super.a()}
  {this}
  {delete a.a}
  {void a}
  {typeof a}
  {+a}
  {-a}
  {!a}
  {~a}
  {``}
  {/* A JSX comment */}
  {/* Multi
      line
  */}
  {}
</div>
function *f() {
    return <div>
        {yield a}
    </div>;
}
