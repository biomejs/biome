/* should not generate diagnostics */
class UsesThisInDefaultParameters {
    defaultHeader = "header";

    print(content, header = this.defaultHeader) {
        console.log(header, content);
    }

    read(callback = () => this.defaultHeader) {
        return callback();
    }

    set header(value = this.defaultHeader) {
        console.log(value);
    }

    arrowField = (value = this.defaultHeader) => value;

    functionField = function (value = this.defaultHeader) {
        return value;
    };
}
