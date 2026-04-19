input.addEventListener("keydown", (e) => {
    if (guardIsComposing(e)) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});
