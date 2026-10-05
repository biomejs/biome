/* should generate diagnostics */
function f(){return(BigInt)(1)}
typeof(BigInt)(1);
void(BigInt)(1);
BigInt(1)in obj;
BigInt(1)instanceof Object;
function* g(){yield(BigInt)(1)}
async function h(){await(BigInt)(1)}
if (condition) {} else(BigInt)(1);
do(BigInt)(1);while (condition);
switch (value) { case(BigInt)(1): break; }
BigInt(1)/* keep */in obj;
