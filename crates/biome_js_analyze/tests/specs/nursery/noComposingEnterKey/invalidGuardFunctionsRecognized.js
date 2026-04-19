input.addEventListener("keypress", (e) => {
    if (guardIsComposing(e)) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keydown", (e) => {
    guardIsComposing(e);

    if (e.key === "Enter") {
        submit();
    }
});
