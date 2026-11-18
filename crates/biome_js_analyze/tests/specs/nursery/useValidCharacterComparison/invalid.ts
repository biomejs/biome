/* should generate diagnostics */
declare const value: string;
value[0] === "ab";
value.at(0) === "ab";
{
	let text: string = getText();
	text[1] === "ab";
}
{
	let text!: string;
	text.at(1) !== "ab";
}
