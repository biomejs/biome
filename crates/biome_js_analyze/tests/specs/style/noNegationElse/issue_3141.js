// invalid
function f() {
    return !!specs.variables ? specs.variables(props) : {};
}
// valid
!-a ? b : c