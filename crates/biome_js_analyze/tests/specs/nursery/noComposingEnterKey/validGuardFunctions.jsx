// should not generate diagnostics

const guarded = <input onKeyDown={(e) => {
    if (guardIsComposing(e)) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
}} />;
