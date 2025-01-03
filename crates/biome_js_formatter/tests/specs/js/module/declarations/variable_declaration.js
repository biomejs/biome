//break left-hand side layout
{
    const {
        id,
        static: isStatic,
        method: isMethod,
        methodId,
        getId,
        setId,
    } = privateNamesMap.get(name);

		// biome-ignore format: test
		const {
			id, static: isStatic, method: isMethod,
			methodId, getId, setId,
		} = privateNamesMap.get(name);

    const { id1, method: isMethod1, methodId1 } = privateNamesMap.get(name);

		const { id1, method: isMethod1, methodId1 } =
			// biome-ignore format: test
			privateNamesMap.get(name);

    const {
        id3,
        method: isMethod3,
        methodId3,
    } = anodyneCondosMalateOverateRetinol.get(bifornCringerMoshedPerplexSawder);

		// biome-ignore format: test
		const {
			id3, method: isMethod3,
			methodId3,
		} =
			// biome-ignore format: test
			anodyneCondosMalateOverateRetinol.get(
			bifornCringerMoshedPerplexSawder
		);
}

//break after operator layout
const loooooooooooooooooooooooooong1 =
	void "looooooooooooooooooooooooooooooooooooooooooog";
const loooooooooooooooooooooooooong2 =
	!"looooooooooooooooooooooooooooooooooooooooooog";
const loooooooooooooooooooooooooong3 =
	+"looooooooooooooooooooooooooooooooooooooooooog";
const loooooooooooooooooooooooooong4 =
	void void "looooooooooooooooooooooooooooooooooooooooooog";
const loooooooooooooooooooooooooong5 =
	!!"looooooooooooooooooooooooooooooooooooooooooog";
// biome-ignore format: test
const   loooooooooooooooooooooooooong6    =
	void    "looooooooooooooooooooooooooooooooooooooooooog";
const loooooooooooooooooooooooooong7    =
	// biome-ignore format: test
	!      "looooooooooooooooooooooooooooooooooooooooooog";

//poorly breakable member or call chain (break after operator layout)
//JsIdentifierExpression
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn();
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect().ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect.ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong4 = objdddddddddectobjdddddddddect[dsadsadsadsadsadsadsa + ewqoewqoeiowqieopwqie];
let loooooooooooooooooooooooooong5 = objdddddddddectobjdddddddddect()[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsThisExpression
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this();
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this().ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this.ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this[dsadsadsadsadsadsadsa + ewqoewqoeiowqieopwqie];
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong5 = this()[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie;

//lone short argument for JsIdentifierExpression
//JsThisExpression argument
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(this);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect(this).ewqeqewqwdddddddddddddeqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(this)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(this)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsIdentifierExpression argument
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(aaaaaaaaaaaaaaaaaaaa);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect(aaaaaaaaaaaaaaaaaaaa).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(aaaaaaaaaaaaaaaaaaaa)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(aaaaaaaaaaaaaaaaaaaa)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsUnaryExpression argument
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(+12312312321321312);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect(-12312312321321312).ewqeqewqweqweqweqweqweqweqw;

//JsStringLiteralExpression argument
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn("111111111111111111");
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect("111111111111111111").ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect("111111111111111111")[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect("111111111111111111")[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsRegexLiteralExpression argument
var loooooooooooooooooooooooooong1 = objdddddddddectobjdddddddddect(/\wwwwwwwwwwwwwwwwww+/gi);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect(/\wwwwwwwwwwwwwwwwww+/gi).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(/\wwwwwwwwwwwwwwwwww+/gi)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(/\wwwwwwwwwwwwwwwwww+/gi)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsTemplate argument
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(``);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddsadsadddddddect(`111111111111111111`).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjddsadsaddddddddect(`111111111111111111`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddsadaddddddddect(`111111111111111111`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

// rest JsAnyLiteralExpression
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(true);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddsadsadasdddect(undefined).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(321321312321312321321)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(9007199254740991n)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie


//lone short argument for JsThisExpression
//JsThisExpression argument
var looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(this);
let looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(this).ewqeqewqweqweqweqweqweqweqw;
let looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(this)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this(this)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsIdentifierExpression argument
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(aaaaaaaaaaaaaaaaaaaa);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(aaaaaaaaaaaaaaaaaaaa).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(aaaaaaaaaaaaaaaaaaaa)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this(aaaaaaaaaaaaaaaaaaaa)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsUnaryExpression argument
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(+12312312321321312);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(-12312312321321312).ewqeqewqweqweqweqweqweqweqw;

//JsStringLiteralExpression argument
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this("111111111111111111");
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this("111111111111111111").ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this("111111111111111111")[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this("111111111111111111")[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsRegexLiteralExpression argument
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(/\wwwwwwwwwwwwwwwwww+/gi);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(/\wwwwwwwwwwwwwwwwww+/gi).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(/\wwwwwwwwwwwwwwwwww+/gi)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this(/\wwwwwwwwwwwwwwwwww+/gi)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsTemplate argument
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(``);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(`111111111111111111`).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(`111111111111111111`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this(`111111111111111111`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

// rest JsAnyLiteralExpression
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(true);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(undefined).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(321321312321312321321)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this(9007199254740991n)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//fluid layout
const otherBrandsWithThisAdjacencyCount123 = Object.values(edge.to.edges).length;
let vgChannel = pointPositionDefaultRef({ model, defaultPos, channel })();
const bifornCringerMoshedPerplexSawderGlyphsHb = someBigFunctionName(
    `foo
`,
)("bar");

//not poorly breakable member or call chain (fluid layout layout)
//JsIdentifierExpression
var looooooooooooooooooooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(1,2,3,4);
let looooooooooooooooooooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect(1,2).ewqeqewqweqweqweqweqweqweqw;
let looooooooooooooooooooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(a,b,c)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//exceed the length of JsIdentifierExpression argument
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(aaaaaaaaaaaaaaaaaaaaa);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect(aaaaaaaaaaaaaaaaaaaaa).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(aaaaaaaaaaaaaaaaaaaaa)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(aaaaaaaaaaaaaaaaaaaaa)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//exceed the length of JsStringLiteralExpression argument
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn("1111111111111111111");
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect("1111111111111111111").ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect("1111111111111111111")[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect("1111111111111111111")[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//exceed the length of JsRegexLiteralExpression argument
var loooooooooooooooooooooooooong1 = objdddddddddectobjdddddddddect(/\wwwwwwwwwwwwwwwwwwwww+/gi);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect(/\wwwwwwwwwwwwwwwwwwwww+/gi).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(/\wwwwwwwwwwwwwwwwwwwww+/gi)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect(/\wwwwwwwwwwwwwwwwwwwww+/gi)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//exceed the length of JsTemplate argument
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(`111111111111111111111111111111111111111111111111111111`);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddsadsadddddddect(`111111111111111111111111111111111111`).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjddsadsaddddddddect(`111111111111111111111111111111111111`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

// has expression
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(`123123 ${adsada} dsa`);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddsadsadddddddect(`123123 ${adsada} dsa`).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjddsadsaddddddddect(`123123 ${adsada} dsa`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

// has new line
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(`123123
 dsa`);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddsadsadddddddect(`123123
dsa`).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooong3 = objdddddddddectobjddsadsaddddddddect(`123123
 dsa`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//JsThisExpression
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(1,2,3);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(a, b).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this()[dsadsadsadsadsadsadsa](g,c,d).ewqoewqoeiowqieopwqie;

//exceed the length of JsIdentifierExpression
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(aaaaaaaaaaaaaaaaaaaaa);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(aaaaaaaaaaaaaaaaaaaaa).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(aaaaaaaaaaaaaaaaaaaaa)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this(aaaaaaaaaaaaaaaaaaaaa)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//exceed the length of JsStringLiteralExpression argument
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this("1111111111111111111");
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this("1111111111111111111").ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this("1111111111111111111")[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this("1111111111111111111")[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie


//exceed the length of JsRegexLiteralExpression argument
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(/\wwwwwwwwwwwwwwwwwwwww+/gi);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(/\wwwwwwwwwwwwwwwwwwwww+/gi).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(/\wwwwwwwwwwwwwwwwwwwww+/gi)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong4 = this(/\wwwwwwwwwwwwwwwwwwwww+/gi)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//exceed the length of JsTemplate argument
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(`111111111111111111111111111111111111111111111111111111`);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(`111111111111111111111111111111111111`).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(`111111111111111111111111111111111111`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

// has expression
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(`123123 ${adsada} dsa`);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(`123123 ${adsada} dsa`).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(`123123 ${adsada} dsa`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

// has new line
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(`123123
 dsa`);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(`123123
dsa`).ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this(`123123
 dsa`)[dsadsadsadsadsadsadsa]().ewqoewqoeiowqieopwqie

//lone short argument JsUnaryExpression argument with comment
var loooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn(+12312312321321312 /*comment*/);
let loooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect(
	//comment
	-12312312321321312).ewqeqewqweqweqweqweqweqweqw;

var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this(+12312312321321312 /*comment*/);
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this(
	//comment
	-12312312321321312).ewqeqewqweqweqweqweqweqweqw;


const a
	// biome-ignore format: Ignore the initializer
				=

			[A,    B,   C].push( aaa )
