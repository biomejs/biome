/* should not generate diagnostics */
{const a = 1, b = 2, c = 3;}
{const a = 1;
const b = 2;
 const c = 3;}
{for(var a = 0, b = 0;;){}}
{for(let a = 0, b = 0;;){}}
{for(const a = 0, b = 0;;){}}
{class C { [foo = 0] = 0 }}
