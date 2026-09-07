/* should generate diagnostics */
element.childNodes[0];
(element.childNodes)[0];
(element.children)[0];
element.children[0];
element.children[1];
element.children[10];
element.children[1].children[2];
element.parentElement.parentElement;
element.parentElement.parentElement.parentElement;
element.querySelector("a").querySelector("b");
document.querySelector("a").querySelector("b");
document.body.querySelector("a").querySelector("b");
element.querySelector('a').querySelector('b');
element.querySelector(`a`).querySelector(`b`);
element.querySelector("a").querySelector(`b`);
element.querySelector("a > b").querySelector(".c");
element.querySelector(".a, .b").querySelector(".c");
element.querySelector(".a").querySelector(".b, .c");
element.querySelector(":scope a").querySelector("b");
element.querySelector(":SCOPE > .item").querySelector("b");
element.querySelector("a").querySelector("b").querySelector("c");
element.querySelector("a").querySelector("b").querySelector(selector);
element.querySelector(selector).querySelector("b").querySelector("c");
(getElement()).querySelector("a").querySelector("b");
(foo || bar).querySelector("a").querySelector("b");

element.childNodes[/* comment */ 0];
element.children[/* comment */ 0];
element.querySelector(/* comment */ "a").querySelector("b");
element.querySelector("a").querySelector(/* comment */ "b");

const item = element
	.children[0];

const nested = element
	.querySelector("a")
	.querySelector("b");
