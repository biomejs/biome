/* should generate diagnostics */
array.flat(2);
array?.flat(2);
array.flat(99,);
array.flat(0b10,);
array.flat(-2);
array.flat(+2);
array.flat(-1.5);
array.flat(-1);
array.flat(0);
array.flat((2));
(array.flat)(2);
array.flat(2) // comment outside the parentheses
/* comment outside the parentheses */ array.flat(2);
