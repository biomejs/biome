const neverBreakAfterColonObject = {
    'this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line-this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line-1': require(),
    x: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
    url: "http://example.com/12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
    "x": "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
    a: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
    ab: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
    abc: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
		'古今': 'https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about',
		a̐éö̲: 'https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about',
    "class-member": class MyClass { constructor() { console.log('class object constructor')}},
    "this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line-this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line-boolean-true": true,
    "this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line-this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line-boolean-false": false,
    "number": 1232132132131231232132112321321321312312321321123213213213123123213211232132132131231232132112321321321312312321321,
    "number-with-dot": 12321321321312312321321123213213213123123213211232132132131231232132112321321321312312321321.12321321321312312321321,
    "template-string": `
    dsadsadas
    32131ewqewq
    `,
};

const breakAfterColonObject = {
    'long-key-for-string': "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
		'古体诗': 'https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about',
	  'logical-expression-1': this.state.longLongLongLongLongLongLongLongLongTooLongProp === true,
    "logical-expression-2": longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337,
    "binary-expression-1": 13321321312312321311332132131231232131232132132132232132132132 + 1332132131231232131232132132132,
    "binary-expression-2": 1332132131231232131232132132132 - 13321321312312321312321321321321332132131231232131232132132132,
    "instanceof-expression": '321321312312ddddddddddddddddddddddd312312312312' instanceof Object,
    "in-expression": {'long-key': '123123213123213123edwqdqwdasdasdsaewqewqewqdas'} in 'long-key',
    "sequence-expression": (33333333333333331, 'dsadsadasdsadas', 3, 'dsadsadasdsadasdsadsadasdsadas', 5),
    'conditional-expression-1': this.state.longLongLongLongLongLongLongLongLongTooLongProp === true ? {} : {},
    'conditional-expression-2': longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337 ? {} : {},
    'conditional-expression-3': 13321321312312321311332132131231232131232132132132232132132132 + 1332132131231232131232132132132 ? {} : {},
    'conditional-expression-4': '321321312312ddddddddddddddddddddddd312312312312' instanceof Object ? {} : {},
    'conditional-expression-5': {'long-key': '123123213123213123edwqdqwdasdasdsaewqewqewqdas'} in 'long-key',

    1: this.state.longLongLongLongLongLlongLongLongLongLongLongLongLongLongTooLongPropongLongLongLongTooLongProp === true,
    2: longLongLongLongLongLongLongLongLonlongLongLongLongLongLongLongLongLongTooLongPropgLongLongLongLongTooLongVar || 1337,
    11: 13321321312312321311332132131231232131232132132132232132132132 + 1332132131231232131232132132132,
    22: 1332132131231232131232132132132 - 13321321312312321312321321321321332132131231232131232132132132,
    33: '321321312312ddddddddddddddddddddddd312312312312' instanceof Object,
    44: {'long-key': '123123213123213123edwqdqwdasdasdsaewqewqewqdas'} in 'long-key',
    5: (33333333333333331, 'dsadsadasdsadas', 3, 'dsadsadasdsadasdsadsadasdsadas', 5),

    a: this.state.longLongLongLongLongLongLongLongLongTooLongProp === true ? {} : {},
    b: longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337 ? {} : {},
    c: 13321321312312321311332132131231232131232132132132232132132132 + 1332132131231232131232132132132 ? {} : {},
    d: '321321312312ddddddddddddddddddddddd312312312312' instanceof Object ? {} : {},
    g: {'long-key': '123123213123213123edwqdqwdasdasdsaewqewqewqdas'} in 'long-key',
    blablah:
        "aldkfkladfskladklsfkladklfkaldfadfkdaf" +
        "adlfasdklfkldsklfakldsfkladsfkadsfladsfa" +
        "dflkadfkladsfklkadlfkladlfkadklfjadlfdfdaf",
};

const fluidObject = {
	'this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line': orMaybeIAmMisunderstandingAndIHaveSetSomethingWrongInMyConfig(),
		"11/01/2017 13:36": [
			{
				message: "test",
				messageType: "SMS",
				status: "Unknown",
				created: "11/01/2017 13:36",
			},
			{
				message: "test",
				messageType: "Email",
				status: "Unknown",
				created: "11/01/2017 13:36",
			},
			{
				message: "te",
				messageType: "SMS",
				status: "Unknown",
				created: "09/01/2017 17:25",
			},
		],
		render: withGraphQLQuery(
			'node(1234567890){image{uri}}',
			function(container, data) {
				return 'image';
			}
		),
    loadNext: (stateIsOK && hasNext) || {
        skipNext: true,
    },
};
