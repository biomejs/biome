/* should generate diagnostics */
x.toString(1 as number);
x.toString(1 satisfies number);
x.toFixed(<number>101);
x.toPrecision(0!);
