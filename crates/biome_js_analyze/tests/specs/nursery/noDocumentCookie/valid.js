document.cookie

const foo = document.cookie;

const array = document.cookie.split("; ");

cookieStore
  .set({
    name: "foo",
    value: "bar",
    expires: Date.now() + 24 * 60 * 60,
    domain: "example.com",
})

function document_is_not_global1(document){
    document.cookie = "bar=foo"
}

function document_is_not_global2(){
    const document = "foo";
    document.cookie = "bar=foo"
}