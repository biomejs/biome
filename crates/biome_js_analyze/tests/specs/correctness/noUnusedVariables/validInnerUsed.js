// https://github.com/biomejs/biome/issues/105

const tid = setInterval(() => {
    clearInterval(tid);
});