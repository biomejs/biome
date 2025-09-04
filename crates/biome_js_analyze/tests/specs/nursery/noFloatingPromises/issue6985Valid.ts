/* should not generate diagnostics */

function doSth() {
    const innerFn = () => {
        return Promise.resolve(1);
    }
    return 'hah'
}

doSth()
