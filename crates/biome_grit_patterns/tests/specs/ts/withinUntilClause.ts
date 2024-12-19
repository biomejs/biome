function delay(timeout: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, timeout);
    });
}

async function unnecessaryAsync(timeout: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, timeout);
    });
}

async function properUseOfAsync() {
    const delay = (timeout: number) => {
        return new Promise((resolve) => {
            setTimeout(resolve, timeout);
        });
    };

    await delay(100);
}
