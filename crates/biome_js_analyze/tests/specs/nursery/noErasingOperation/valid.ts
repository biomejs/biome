/* should not generate diagnostics */
x / (0 as number);
0 / (0 satisfies number);
x * (1 as number);
(0n as bigint) * x;
