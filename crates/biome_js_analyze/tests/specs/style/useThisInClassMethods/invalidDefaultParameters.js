class DefaultParametersWithoutThis {
    print(content, header = "header") {
        console.log(header, content);
    }

    read(callback = function () {
        return this.value;
    }) {
        return callback();
    }

    arrowField = (value = 1) => value;
}
