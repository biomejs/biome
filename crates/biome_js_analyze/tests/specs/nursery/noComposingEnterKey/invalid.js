input.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keyup", (e) => {
    if (e.key === "Enter") {
        submit();
    }
});

el.addEventListener("keydown", (e) => {
    if ("Enter" === e.key) {
        go();
    }
});

input.addEventListener("keydown", (e) => {
    if (e.code === "Enter") {
        submit();
    }
});

input.addEventListener("keydown", (e) => {
    if (e.keyCode === 13) {
        submit();
    }
});

input.addEventListener("keydown", function (e) {
    if (e.which == 13) {
        send();
    }
});

input.addEventListener("keyup", function (e) {
    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keydown", async (e) => {
    if (e.key === "Enter") {
        await submit();
    }
});

el.addEventListener("keydown", (e) => {
    if (e.key === "Enter" && !e.shiftKey) {
        go();
    }
});

input.addEventListener("keydown", (e) => {
    e.key === "Enter" ? submit() : null;
});

input.addEventListener("keydown", (e) => {
    if (e.key == "Enter") {
        submit();
    }
});

input.addEventListener("keydown", (e) => {
    const composing = e.isComposing;
    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keydown", (e) => {
    switch (e.key) {
        case "Enter":
            submit();
            break;
    }
});

input.addEventListener("keydown", (e) => {
    switch (e.code) {
        case "Enter":
            submit();
            break;
    }
});

input.addEventListener("keydown", (e) => {
    switch (e.keyCode) {
        case 13:
            submit();
            break;
    }
});

input.addEventListener("keydown", (e) => {
    switch (e.which) {
        case 13:
            submit();
            break;
    }
});

input.onkeydown = (e) => {
    if (e.key === "Enter") {
        submit();
    }
};

input.onkeydown = function (e) {
    if (e.key === "Enter") {
        submit();
    }
};

input.onkeydown = (e) => {
    if (e.code === "Enter") {
        submit();
    }
};

input.onkeydown = (e) => {
    switch (e.key) {
        case "Enter":
            submit();
            break;
    }
};

input.onkeyup = (e) => {
    if (e.key === "Enter") {
        submit();
    }
};

input.onkeyup = (e) => {
    if (e.code === "Enter") {
        submit();
    }
};

window.onkeydown = (e) => {
    if (e.key === "Enter") {
        submit();
    }
};

input.addEventListener("keypress", (e) => {
    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keypress", (e) => {
    if (e.keyCode === 13) {
        submit();
    }
});

input.addEventListener("keypress", (e) => {
    if (e.isComposing) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keydown", (e) => e.key === "Enter" && submit());
input.addEventListener("keypress", (e) => e.key === "Enter" && submit());

input.onkeyup = (e) => {
    switch (e.key) {
        case "Enter":
            submit();
            break;
    }
};

input.onkeydown = (e) => {
    if (e.keyCode === 13) {
        submit();
    }
};

input.onkeydown = (e) => {
    if (e.which == 13) {
        submit();
    }
};

input.onkeyup = (e) => {
    if (e.keyCode === 13) {
        submit();
    }
};

input.onkeyup = (e) => {
    if (e.which == 13) {
        submit();
    }
};

input.onkeypress = (e) => {
    if (e.code === "Enter") {
        submit();
    }
};

input.onkeypress = (e) => {
    if (e.keyCode === 13) {
        submit();
    }
};

input.addEventListener("keydown", (e) => {
    if (e.key !== "Enter") {
        return;
    }

    submit();
});

input.addEventListener("keydown", (e) => {
    if (e.keyCode != 13) {
        return;
    }

    submit();
});

input.addEventListener("keyup", (e) => {
    if (e.key !== "Enter") {
        return;
    }

    submit();
});

input.addEventListener("keypress", (e) => {
    if (e.key !== "Enter") {
        return;
    }

    submit();
});

input.onkeydown = (e) => {
    if (e.key !== "Enter") {
        return;
    }

    submit();
};

input.onkeyup = (e) => {
    if (e.key !== "Enter") {
        return;
    }

    submit();
};

input.addEventListener("keydown", (e) => {
    e.isComposing ? null : (e.key === "Enter" && submit());
});

input.onkeypress = (e) => {
    if (e.isComposing) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
};

input.addEventListener("keypress", (e) => {
    switch (e.key) {
        case "Enter":
            submit();
            break;
    }
});

input.addEventListener("keypress", (e) => {
    switch (e.code) {
        case "Enter":
            submit();
            break;
    }
});

input.addEventListener("keypress", (e) => {
    switch (e.keyCode) {
        case 13:
            submit();
            break;
    }
});

input.addEventListener("keypress", (e) => {
    switch (e.which) {
        case 13:
            submit();
            break;
    }
});

input.onkeypress = (e) => {
    switch (e.key) {
        case "Enter":
            submit();
            break;
    }
};

input.onkeypress = (e) => {
    switch (e.keyCode) {
        case 13:
            submit();
            break;
    }
};

input.addEventListener("keyup", (e) => {
    switch (e.code) {
        case "Enter":
            submit();
            break;
    }
});

input.onkeyup = (e) => {
    switch (e.code) {
        case "Enter":
            submit();
            break;
    }
};

input.onkeydown = (e) => e.key === "Enter" && submit();
input.onkeyup = (e) => e.key === "Enter" && submit();
input.onkeypress = (e) => e.key === "Enter" && submit();

input.addEventListener("keydown", (e) => {
    if (e.isComposing) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});

input.addEventListener("keyup", (e) => {
    if (e.isComposing) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
});

input.onkeydown = (e) => {
    if (e.isComposing) {
        return;
    }

    if (e.key === "Enter") {
        submit();
    }
};

input.addEventListener("keydown", (e) => {
    if (e.isComposing) {
        return;
    }

    switch (e.key) {
        case "Enter":
            submit();
            break;
    }
});

input.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
        submit();
    }
});

window.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
        submit();
    }
});

window.addEventListener("keyup", (e) => {
    if (e.key === "Enter") {
        submit();
    }
});

window.addEventListener("keypress", (e) => {
    if (e.key === "Enter") {
        submit();
    }
});
