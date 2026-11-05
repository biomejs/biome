// should generate diagnostics
declare const arr: number[]
declare function getValue(): unknown

// No fix is offered, because the statement would continue the previous line.
getValue()
delete (arr)[0]
getValue()
delete (arr as number[])[0]

// The fix is offered, because the previous statement ends here.
{
	delete (arr)[0]
}
