/* should not generate diagnostics */

// Already preferred APIs
element.firstChild;
element.firstElementChild;
element.querySelector("li");
element.closest("form");

// Not a MemberExpression with a receiver
children[0];
parentElement.parentElement;

// Non-literal and unsupported indexes
element.children[index];
element.children["0"];
element.children[-1];
element.children[1.5];
element.childNodes[index];
element.childNodes[1];

// Computed collection names
element["children"][0];
element[children][0];
element["childNodes"][0];
element[childNodes][0];
element["parentElement"].parentElement;
element.parentElement["parentElement"];

// Optional chaining
element?.children[0];
element.children?.[0];
element?.parentElement.parentElement;
element.parentElement?.parentElement;
element.querySelector?.("a").querySelector("b");
element.querySelector("a")?.querySelector("b");
element.querySelector?.("a").querySelector("b").querySelector("c");
element.querySelector("a")?.querySelector("b").querySelector("c");
element.querySelector("a").querySelector("b")?.querySelector("c");
element.querySelector("a").querySelector("b")?.foo;
element.querySelector("a").querySelector("b").foo?.querySelector("c");
(element?.querySelector("a")).querySelector("b").querySelector("c");

// Wrong selector methods or arguments
element.querySelector();
element.querySelector("a", root).querySelector("b");
element.querySelector("a").querySelector();
element.querySelector("a").querySelector("b", root);
element.querySelectorAll("a").querySelector("b");
element.querySelector("a").querySelectorAll("b");

// Non-static selectors
element.querySelector(selector).querySelector("b");
element.querySelector("a").querySelector(selector);
element.querySelector(`${selector}`).querySelector("b");
element.querySelector("a").querySelector(`${selector}`);
element.querySelector(tag`a`).querySelector("b");

// Receiver cannot be a DOM node
([]).children[0];
([element]).children[0];
([...elements]).children[0];
(() => {}).children[0];
(class Node {}).children[0];
(function () {}).children[0];
(0).children[0];
(1).children[0];
(0.1).children[0];
("").children[0];
("string").children[0];
(/regex/).children[0];
(null).children[0];
(0n).children[0];
(1n).children[0];
(true).children[0];
(false).children[0];
({}).children[0];
(`templateLiteral`).children[0];
(undefined).children[0];

([]).childNodes[0];
(0).childNodes[0];
("string").childNodes[0];
(null).childNodes[0];
(undefined).childNodes[0];

([]).parentElement.parentElement;
(0).parentElement.parentElement;
(null).parentElement.parentElement;
(undefined).parentElement.parentElement;

([]).querySelector("a").querySelector("b");
(0).querySelector("a").querySelector("b");
("string").querySelector("a").querySelector("b");
(null).querySelector("a").querySelector("b");
(undefined).querySelector("a").querySelector("b");

// Component props.children is not DOM traversal
const fromProps = props.children[0];
const fromNestedProps = component.props.children[0];
const laterChild = props.children[1];
