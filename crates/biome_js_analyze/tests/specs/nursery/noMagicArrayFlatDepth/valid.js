/* should not generate diagnostics */
array.flat(1);
array.flat(1.0);
array.flat(0x01);
array.flat(unknown);
array.flat(Number.POSITIVE_INFINITY);
array.flat(Infinity);
array.flat(/* explanation */2);
array.flat(2/* explanation */);
array.flat(
	// explanation
	2
);
array.flat(
	2, // explanation
);
array.flat();
array.flat(2, extraArgument);
array.flat(...depths);
new array.flat(2);
array.flat?.(2);
array.notFlat(2);
array["flat"](2);
flat(2);
array.flat(+1);
array.flat(-(-1));
array.flat(2n);
array.flat("2");
array.flat((unknown, 2));
