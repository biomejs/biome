const keyDown = <input onKeyDown={(e) => {
    if (e.key === "Enter") {
        submitForm();
    }
}} />;

const keyDownWhich = <input onKeyDown={(e) => {
    if (e.which === 13) {
        submit();
    }
}} />;

const keyDownCode = <input onKeyDown={(e) => {
    if (e.code === "Enter") {
        submitForm();
    }
}} />;

const keyDownKeyCode = <input onKeyDown={(e) => {
    if (e.keyCode === 13) {
        submitForm();
    }
}} />;

const keyDownSwitch = <input onKeyDown={(e) => {
    switch (e.key) {
        case "Enter":
            submitForm();
            break;
    }
}} />;

const keyDownEarlyReturn = <input onKeyDown={(e) => {
    if (e.key !== "Enter") {
        return;
    }

    submit();
}} />;

const keyDownConcise = <input onKeyDown={(e) => e.key === "Enter" && submit()} />;
const keyDownFunction = <input onKeyDown={function (e) {
    if (e.key === "Enter") {
        submitForm();
    }
}} />;

const keyDownSafari = <input onKeyDown={(e) => {
    if (e.isComposing) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
}} />;

const keyUp = <input onKeyUp={(e) => {
    if (e.key === "Enter") {
        submitForm();
    }
}} />;

const keyUpKeyCode = <input onKeyUp={(e) => {
    if (e.keyCode === 13) {
        submitForm();
    }
}} />;

const keyUpSwitch = <input onKeyUp={(e) => {
    switch (e.key) {
        case "Enter":
            submitForm();
            break;
    }
}} />;

const keyUpEarlyReturn = <input onKeyUp={(e) => {
    if (e.key !== "Enter") {
        return;
    }

    submit();
}} />;

const keyUpConcise = <input onKeyUp={(e) => e.key === "Enter" && submit()} />;

const keyPress = <input onKeyPress={(e) => {
    if (e.key === "Enter") {
        submitForm();
    }
}} />;

const keyPressIsComposing = <input onKeyPress={(e) => {
    if (e.isComposing) {
        return;
    }

    if (e.key === "Enter") {
        submitForm();
    }
}} />;

const keyPressKeyCode = <input onKeyPress={(e) => {
    if (e.keyCode === 13) {
        submit();
    }
}} />;

const keyPressWhich = <input onKeyPress={(e) => {
    if (e.which === 13) {
        submit();
    }
}} />;

const keyPressSwitchKey = <input onKeyPress={(e) => {
    switch (e.key) {
        case "Enter":
            submitForm();
            break;
    }
}} />;

const keyPressSwitchCode = <input onKeyPress={(e) => {
    switch (e.code) {
        case "Enter":
            submit();
            break;
    }
}} />;

const keyPressSwitchKeyCode = <input onKeyPress={(e) => {
    switch (e.keyCode) {
        case 13:
            submit();
            break;
    }
}} />;

const keyPressConcise = <input onKeyPress={(e) => e.key === "Enter" && submit()} />;
const keyPressFunction = <input onKeyPress={function (e) {
    if (e.key === "Enter") {
        submitForm();
    }
}} />;
