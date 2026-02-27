

const returnsObjectArrow = () => ( { a: 1, b: 2 });


const returnsSequenceArrow = () => {
    return (a, b);
}


const returnsAwaitArrow = async () => {
    return await fetchData();
}

const nestedArrow = {
    method: () => {
        const inner = () => ({ a: 1 });
        return inner;
    }
};


