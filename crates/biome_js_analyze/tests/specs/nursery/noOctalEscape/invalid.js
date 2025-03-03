0;
"foo \01 bar";
"foo \000 bar";
"foo \377 bar";
"foo \378 bar";
"foo \37a bar";
"foo \381 bar";
"foo \3a1 bar";
"foo \751 bar";
"foo \258 bar";
"foo \25a bar";

const o = {
    '\31': 0,
};
