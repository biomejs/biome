// should not generate diagnostics

input.addEventListener("keydown", (e) => {
    if (guardIsComposing(e)) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});
