a  =    b
a  +=   b
a  -=   b
a  *=   b
a  /=   b
a  %=   b
a  <<=  b
a  >>=  b
a  >>>= b
a  &=   b
a  |=    b
a  ^=   b
a  &&=  b
a  ||=  b
a  ??=  b
a  **=  b
a.b  =  c.#d
a[ b ]  =  c[ d ]
;( a )  =  b
;[a, b = "b", ...c] = d
;[fooooooooooooooooooooooooooooooooooooooooooooooooo, barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr, bazzzzzzzzzzzzzzzzzzzzzzzzzz] = d
;({a,b=c,d:e,f:g=h,...j} = x)
;({aaaaaaaaaa,bbbbbbbbbb=cccccccccc,dddddddddd:eeeeeeeeee,ffffffffff:gggggggggg=hhhhhhhhhh,...jjjjjjjjjj} = x);

(s||(s=Object.create(null)))[i]=!0;
(s||(s=Object.create(null))).test=!0;

//never break after operator object layout
this_is_a_very_long_key_and_the_assignment_should_be_put_on_the_next_line_this_is_a_very_long_key_and_the_assignment_should_be_put_on_the_next_line_1 = require();
class_member_with_looooooooooooooooongggggggg_nameeeeeeeeee = class MyLooooonnnngggClassNamee { constructor() { console.log('class object constructor')}};
this_is_a_very_long_key_and_the_assignment_should_be_put_on_the_next_line_this_is_a_very_long_key_and_the_assignment_should_be_put_on_the_next_line_boolean_true = true;
this_is_a_very_long_key_and_the_assignment_should_be_put_on_the_next_line_this_is_a_very_long_key_and_the_assignment_should_be_put_on_the_next_line_boolean_false = false;
number = 1232132132131231232132112321321321312312321321123213213213123123213211232132132131231232132112321321321312312321321;
number_with_dot = 12321321321312312321321123213213213123123213211232132132131231232132112321321321312312321321.12321321321312312321321;
template_string = `
    dsadsadas
    32131ewqewq
    `;

//break after operator layout
x = "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
url = "http://example.com/12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
a = "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
ab = "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
abc = "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
long_key_for_string = "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890";
古体诗 = 'https://prettier.io/docs/en/rationale.html#what_prettier_is_concerned_about';
logical_expression_1 = this.state.longLongLongLongLongLongLongLongLongTooLongProp === true;
logical_expression_2 = longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337;
binary_expression_1 = 13321321312312321311332132131231232131232132132132232132132132 + 1332132131231232131232132132132;
binary_expression_2 = 1332132131231232131232132132132 + 13321321312312321312321321321321332132131231232131232132132132;
instanceof_expression = '321321312312ddddddddddddddddddddddd312312312312' instanceof Object;
in_expression = {'long_key': '123123213123213123edwqdqwdasdasdsaewqewqewqdas'} in 'long_key';
sequence_expression = (33333333333333331, 'dsadsadasdsadas', 3, 'dsadsadasdsadasdsadsadasdsadas', 5);
conditional_expression_1 = this.state.longLongLongLongLongLongLongLongLongTooLongProp === true ? {} : {};
conditional_expression_2 = longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337 ? {} : {};
conditional_expression_3 = 13321321312312321311332132131231232131232132132132232132132132 + 1332132131231232131232132132132 ? {} : {};
conditional_expression_4 = '321321312312ddddddddddddddddddddddd312312312312' instanceof Object ? {} : {};
conditional_expression_5 = {'long_key': '123123213123213123edwqdqwdasdasdsaewqewqewqdas'} in 'long_key';
a = this.state.longLongLongLongLongLongLongLongLongTooLongProp === true ? {} : {};
b = longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337 ? {} : {};
c = 13321321312312321311332132131231232131232132132132232132132132 + 1332132131231232131232132132132 ? {} : {};
d = '321321312312ddddddddddddddddddddddd312312312312' instanceof Object ? {} : {};
g = {'long_key': '123123213123213123edwqdqwdasdasdsaewqewqewqdas'} in 'long_key';
blablah =
    "aldkfkladfskladklsfkladklfkaldfadfkdaf" +
    "adlfasdklfkldsklfakldsfkladsfkadsfladsfa" +
    "dflkadfkladsfklkadlfkladlfkadklfjadlfdfdaf";
fn =
// something
    fn()

loooooooooooooooooooooooooong1 =
	void "looooooooooooooooooooooooooooooooooooooooooog";
loooooooooooooooooooooooooong2 =
	!"looooooooooooooooooooooooooooooooooooooooooog";
loooooooooooooooooooooooooong3 =
	+"looooooooooooooooooooooooooooooooooooooooooog";
loooooooooooooooooooooooooong4 =
	void void "looooooooooooooooooooooooooooooooooooooooooog";
loooooooooooooooooooooooooong5 =
	!!"looooooooooooooooooooooooooooooooooooooooooog";
// biome-ignore format: test
loooooooooooooooooooooooooong6       =
	void    "looooooooooooooooooooooooooooooooooooooooooog";
loooooooooooooooooooooooooong7    =
	// biome-ignore format: test
	!     "looooooooooooooooooooooooooooooooooooooooooog";

// fluid layout
bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers = [1, 2, 3, 4, 5];
bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers = { a: 10 };
bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder = bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawderArrayNumbes = { a: 10 };
fn =

    fn()
this_is_a_very_long_key_and_the_assignment_should_be_put_on_the_next_line = orMaybeIAmMisunderstandingAndIHaveSetSomethingWrongInMyConfig();
dsakdljaskldjaslk = [
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
];
render =  withGraphQLQuery(
    'node(1234567890){image{uri}}',
    function(container, data) {
        return 'image';
    }
);

loadNext = (stateIsOK && hasNext) || {
    skipNext: true,
};

// chain and chain tail where it breaks
bifornCringerMoshedPerplex = bifornCringerMoshedPerplexSawder = arrayOfNumb = a = "test"

// chain and chain tail where it doesn't break
loreum = ipsum = arrayOfNumb = a = "test"

// chain tail arrow function
lorem = fff = ee = () => (fff) => () => (fefef) => () => fff;

// complex destructuring, break left hand
a = {
    a: { t: c = b }, loreum, ipsurm
} = {}

a =
// biome-ignore format: test
{
    a: { t: c = b }, loreum, ipsurm
} = {}

a =
    {
// biome-ignore format: test
        a: { t: c = b       }, loreum, ipsurm
    } = {}


const { looooooooooooooooooooooooooooooooooooooooooooongValue } =
	await setup(2);

function* f() {
	const { looooooooooooooooooooooooooooooooooooooooooooongValue } =
		yield setup(2);
}

export const MSG_WITH_REMOVED_ESCAPE_CHARACTER_TEST =
  goog.getMsg("That\'s all we know");

export const MSG_WITHOUT_ESCAPE_CHARACTER_TEST = goog.getMsg(
	"That's all we know",
);
