// =====================
// Logical not operator
// =====================

// Basic logical not with identifier
!a;

// Logical not with literal values
!true;
!false;
!0;

// Double logical not (no space between the two !s)
!!a;
!!true;

// Triple logical not
!!!a;

// Quadruple logical not
!!!!a;

// Logical not with function call
!foo();

// Logical not with member expression
!foo.bar;

// Logical not chained with other unary operators
!-a;
-!a;
!+a;
+!a;
!~a;
~!a;

// Logical not with parenthesized expression
!(a || b);

// Trailing comment on expression
!a /* comment */;

// Multiple comments around expression
!/* before */ a /* after */;

// =====================
// Contrast: other unary operators (no space added)
// =====================

-a;
+a;
~a;
typeof a;
void a;

// =====================
// Boundary tests
// =====================

// Boundary: 79 chars without space, 80 with space = fits on one line
!aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;

// Boundary: 80 chars without space, 81 with space = exceeds line width
// (atomic identifiers don't break, they just exceed)
!aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;
