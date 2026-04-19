// should not generate diagnostics

const form = <form onSubmit={(e) => {
    e.preventDefault();
    send();
}} />;

const guardedKeyDown = <input onKeyDown={(e) => {
    if (e.isComposing || e.keyCode === 229) {
        return;
    }

    if (e.key === "Enter") {
        submitForm();
    }
}} />;

const guardedKeyUp = <input onKeyUp={(e) => {
    if (e.isComposing || e.keyCode === 229) {
        return;
    }

    if (e.key === "Enter") {
        submitForm();
    }
}} />;

const keyPressEscape = <input onKeyPress={(e) => {
    if (e.key === "Escape") {
        close();
    }
}} />;

const namedReference = <input onKeyDown={handleKeydown} />;
