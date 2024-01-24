var a = x ? true : true;
var a = x ? false : false;

var a = foo() ? false : true;
var a = foo ? false : true;
var a = foo === 1 ? false : true;
var a = foo + 1 ? false : true;

var a = foo() ? true : false;
var a = foo ? true : false;
var a = foo === 1 ? true : false;
var a = foo + 1 ? true : false;

var a= !foo? true : false;

var a = x instanceof foo ? false : true;
var a = x instanceof foo ? true : false;

var a = 'make' in car ? true : false;
var a = 'make' in car ? false : true;