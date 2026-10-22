<script>
/* should generate diagnostics */
export default {
  props: {
    myProp: 'Number',
    anotherType: ['Number', 'String'],
    extraProp: {
      type: 'Number',
      default: 10,
    },
    lastProp: {
      type: ['Boolean'],
    },
    nullProp: 'null',
    keyword: 'class',
    awaitKeyword: 'await',
    template: `String`,
    binary: Foo + '',
    number: 1000,
    bigInt: 10n,
    boolean: true,
    regex: /a/,
    emptyStr: '',
    emptyTemplate: ``,
    notIdentifier: 'a b',
    withSubstitution: `${Foo}`,
    parenthesized: ('String'),
    update: i++,
    preUpdate: --i,
    inOperator: 'a' in b,
    instanceofOperator: a instanceof B,
    withHoles: ['String',,],
  },
};
</script>
