<>
    <a href="foobar"></a>
    <a randomTag></a>
    <a target />
    <a href="foobar" target="_blank" rel="noopener noreferrer"></a>
    <a href="foobar" target="_blank" rel={"noopener noreferrer"}></a>
    <a href="foobar" target="_blank" rel="noreferrer"></a>
    <a href="foobar" target="_blank" rel={"noreferrer"}></a>
    <a href={"foobar"} target={"_blank"} rel={"noreferrer"}></a>
    <a href={'foobar'} target={'_blank'} rel={'noopener noreferrer'}></a>
    <a href={'foobar'} target={'_blank'} rel={'noreferrer'}></a>
    <a href={`foobar`} target={`_blank`} rel={`noopener noreferrer`}></a>
    <a href={`foobar`} target={`_blank`} rel={`noreferrer`}></a>
    <a target="_blank" {...spreadProps} rel="noopener noreferrer"></a>
    <a target="_blank" {...spreadProps} rel="noreferrer"></a>
    <a target="_blank" href="//example.com" rel={getRel()}></a>
    <a target="_blank" href="//example.com" rel={relValue}></a>
    <a {...spreadProps} target="_blank" rel="noopener noreferrer" href="https://example.com">s</a>
    <a {...spreadProps} target="_blank" rel="noreferrer" href="https://example.com">s</a>
    <a target="_blank" rel="noopener noreferrer" {...spreadProps}></a>
    <a target="_blank" rel="noreferrer" {...spreadProps}></a>
    <a target="_blank" rel={relValue}></a>
    <a target={targetValue} rel="noopener noreferrer"></a>
    <a target={targetValue} rel="noreferrer"></a>
    <a target={targetValue} rel={"noopener noreferrer"}></a>
    <a target={targetValue} rel={"noreferrer"}></a>
    <a target={targetValue} href="relative/path"></a>
    <a target={targetValue} href="/absolute/path"></a>
    <a target={'targetValue'} href="/absolute/path"></a>
    <a target={"targetValue"} href="/absolute/path"></a>
    <a target={null} href="//example.com"></a>
    <Link href="foobar" target="_blank" />
</>
