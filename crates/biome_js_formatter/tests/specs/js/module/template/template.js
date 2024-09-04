`something`;

tag`something`
`something ${ " hello" }`;

`something ${ () => { var hey; const looooooooooong_expression = "loooooooooong_expression" }} something else ${ ehy }`;
    `something ${ () => { var hey; const looooooooooong_expression = "loooooooooong_expression"; return hey; }} something else ${ ehy }`;


`test
  abcd ${input}
output
`;

`test
  abcd ${ () => { var hey; const looooooooooong_expression = "loooooooooong_expression"; return hey; }}
output
`;

// Single Line
const bar =`but where will ${this.fanta} wrap ${baz} ${"hello"} template literal? ${bar.ff.sss} long long long long ${foo[3]} long long long long long long`;


// Fit
const foo = `but where will ${a && b && bar || c && d && g} wrap long long long long long long`;

const foo = `but where will ${lorem && loremlorem && loremlorem || loremc && lorem && loremlorem} wrap long long long long long long`;

const a = `
let expression_is_simple = is_plain_expression(&expression)?;
${loooooong || loooooong || loooooong || loooooong || loooooong || loooooong || loooooong || loooooong }
let expression_is_simple = is_plain_expression(&expression)?;
`;

const foo = `but where will ${
    // with comment
    this.fanta} wrap long long long long long long`;

`<div>${this.set && this.set.artist
    /* avoid console errors if `this.set` is undefined */}</div>`;

`<div>${ /* avoid console errors if `this.set` is undefined */
    this.set && this.set.artist}</div>`;

`${// $FlowFixMe found when converting React.createClass to ES6
ExampleStory.getFragment('story')}
`;

// https://github.com/biomejs/biome/issues/2470
let message = `this is a long message which contains an interpolation: ${format(data)} <- like this`;

let otherMessage = `this template contains two interpolations: ${this(one)}, which should be kept on its line,
and this other one: ${this(long.placeholder.text.goes.here.so.we.get.a.linebreak)
  }
which already had a linebreak so can be broken up
`;

message = `this is a long messsage a simple interpolation without a linebreak \${foo} <- like this\`;
message = \`whereas this messsage has a linebreak in the interpolation \${
  foo
} <- like this`;

// https://github.com/biomejs/biome/issues/3766
const issus_3766 = `Lectures: ${doneCount}/${totalCount} | Mins: ${(doneSecs / 60).toFixed(
    2,
  )}/${(totalSecs / 60).toFixed(
    2,
  )}| Hours: ${(doneSecs / 3600).toFixed(
    2,
  )}/${(totalSecs / 3600).toFixed(
    2,
  )}`;