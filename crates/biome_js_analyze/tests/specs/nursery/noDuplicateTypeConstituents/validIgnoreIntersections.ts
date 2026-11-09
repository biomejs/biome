/* should not generate diagnostics */
type V1 = A & A;
type V2 = (A & B) & (A & B);
