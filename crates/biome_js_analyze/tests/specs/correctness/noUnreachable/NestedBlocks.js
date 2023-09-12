function Outer() {
    label: {
        function Inner() {
            label2: {
                break label2;
            };
        }
    };
}
