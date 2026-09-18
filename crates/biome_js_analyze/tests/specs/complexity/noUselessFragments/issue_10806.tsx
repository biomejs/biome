/* should generate diagnostics */
const cases = (
    <>
        <Component
            overlays={
                <>
                    <div className="overlay" />
                </>
            }
        />
        <Component nested={<><><span /></></>} />
        <Component expression={<>{value}</>} />
        <Component emptyExpression={<>{}</>} />
    </>
);
