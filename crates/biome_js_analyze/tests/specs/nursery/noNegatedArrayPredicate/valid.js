/* should not generate diagnostics */
array.some(element => test(element));
array.every(element => test(element));
!array.some(Boolean);
!array.every(Boolean);
!array.some(predicate);
!array["some"](element => test(element));
!array.some(...predicates);
!array.some(element => test(element), ...rest);
!array.some();
!array.some(element => test(element), thisArgument, extra);
!array?.some(element => test(element));
!array.some?.(element => test(element));
!foo?.array.some(element => test(element));
!"foo".some(element => test(element));
!`foo`.some(element => test(element));
!new Collection().some(element => test(element));
!new Set().some(element => test(element));
!{}.some(element => test(element));
!array.some(element => { return; });
!array.some(element => { if (foo) { return true; } return test(element); });
!array.some(element => { test(element); });
!array.some(async element => test(element));
!array.some(function * (element) { return test(element); });
!array.find(element => test(element));
!array.#some(element => test(element));
-array.some(element => test(element));
const set = new Set();
!set.some(element => test(element));
function* generator() {
	yield !array.some;
}
