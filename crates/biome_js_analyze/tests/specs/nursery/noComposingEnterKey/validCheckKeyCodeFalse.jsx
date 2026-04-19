// should not generate diagnostics

const keyDown = <input onKeyDown={(e) => {
    if (e.isComposing) {
        return;
    }

    if (e.key === "Enter") {
        submitForm();
    }
}} />;

const keyUp = <input onKeyUp={(e) => {
    if (e.isComposing) {
        return;
    }

    if (e.key === "Enter") {
        submitForm();
    }
}} />;
