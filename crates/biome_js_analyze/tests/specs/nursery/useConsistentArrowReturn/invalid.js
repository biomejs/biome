const returnsObjectArrow = () => {
    return { a: 1, b: 2 };
}

const returnsSequenceArrow = () => {
    return (a, b);
}


const returnsAwaitArrow = async () => {
    return await fetchData();
}
