document.cookie = "foo=bar";
document.cookie += ";foo=bar"

window.document.cookie = "foo=bar";
globalThis.window.document.cookie = "foo=bar";

document["cookie"] = "foo=bar"

function document_is_global1(){
    const doc = document;
    doc.cookie = "bar=foo"
}

function document_is_global2(){
    const foo = window.document;
    const bar = foo;
    bar.cookie = "foo=bar";
}

const global_doc = globalThis.document;
function document_is_global3(){
    global_doc.cookie = "foo=bar";
}