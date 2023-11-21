const a = <div></div>;

<Foo
    className={style}
    reallyLongAttributeName1={longComplexValue}
    reallyLongAttributeName2={anotherLongValue}
/>;

<Foo
    className={style}
    reallyLongAttributeName1={longComplexValue}
    reallyLongAttributeName2={anotherLongValue}
>
    Hi
</Foo>;

<div className="hi" />;
<div className="hi"></div>;