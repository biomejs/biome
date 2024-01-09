function a() { // trailing comment
    let a = 2;


 /** leading comment **/   }


function b() // leading comment
{ // trailing


}

function c( //some comment
    foo, bar,
) {}


(function d()
// a
{
  return 42
});

function e()
// a
{
  ;
};

function f()
// a
{
  a;
};

function h() /* a */ {
	a;
};
