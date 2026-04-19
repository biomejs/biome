// should not generate diagnostics

input.addEventListener("keydown", (e) => {
    if (guardIsComposing(e)) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keyup", (e) => {
    if (guardIsComposing(e)) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});

input.onkeydown = (e) => {
    if (guardIsComposing(e)) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
};

input.addEventListener("keydown", (e) => {
    if (isComposingGuard(e)) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keydown", (e) => {
    if (!guardIsComposing(e)) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keydown", (e) => {
    if (guardIsComposing(e) && !e.shiftKey) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});
