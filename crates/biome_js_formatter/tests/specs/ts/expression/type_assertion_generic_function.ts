var a = < <T>(x: T) => T>f;
var b = <(<T>() => void)>f;
var c = < <T>(x: T) => T>((x) => {
	return null;
});
var d = <() => void>f;
var e = <new <T>() => T>f;
var g = <Map<string, <T>() => T>>f;
