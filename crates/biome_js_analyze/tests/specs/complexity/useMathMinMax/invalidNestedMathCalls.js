/* should generate diagnostics */

Math.min(a, b) < c ? Math.min(a, b) : c;
Math.max(a, b) > c ? Math.max(a, b) : c;
c < Math.min(a, b) ? c : Math.min(a, b);
Math.max(a, b) <= c ? c : Math.max(a, b);
Math.min(Math.max(a, b), c) > d ? d : Math.min(Math.max(a, b), c);
