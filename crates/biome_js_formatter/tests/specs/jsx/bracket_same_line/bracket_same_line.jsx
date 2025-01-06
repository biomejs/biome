const a = <div></div>;

<Foo
	className={style}
	reallyLongAttributeName1={longComplexValue}
	reallyLongAttributeName2={anotherLongValue}
/>;

<Foo
		className={style}
		reallyLongAttributeName1={longComplexValue}
		reallyLongAttributeName2={anotherLongValue} // comment
/>;

<Foo
	className={style}
	reallyLongAttributeName1={longComplexValue}
	reallyLongAttributeName2={anotherLongValue}
>
	Hi
</Foo>;

<Foo
		className={style}
		reallyLongAttributeName1={longComplexValue}
		reallyLongAttributeName2={anotherLongValue}
		// comment
>
	Hi
</Foo>;

<Foo
	className={style}
	reallyLongAttributeName1={longComplexValue}
	reallyLongAttributeName2={anotherLongValue}
	// comment
	reallyLongAttributeName3={yetAnotherLongValue}
>
	Hi
</Foo>;

<div className="hi" />;
<div className="hi"></div>;
