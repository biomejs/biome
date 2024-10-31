// SCRIPT
function f() {
    let a = <div>a</div>; // JSX
    let b = <string>b; // type assertion
    let c = <string>b<a>d; // type assertion
    let d = <div>a</div>/; // ambiguous: JSX or "type assertion a less than regex /div>/". Probably JSX.
    let d = <string>a</string>/;
}
