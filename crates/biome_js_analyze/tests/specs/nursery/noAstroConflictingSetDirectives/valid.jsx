// should not generate diagnostics

const elements = (
    <>
        <div set:html={html} set:text={text}>text child</div>
        <Component set:html={first} set:html={second} />
    </>
);
