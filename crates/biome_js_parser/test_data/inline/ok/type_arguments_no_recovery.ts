for (let i = 0 ; i < 3; ++i) {
    verify.completions({
        marker: `${i + 1}`,
        exact: [
            { name: "foo", replacementSpan: test.ranges()[i] },
            { name: "bar", replacementSpan: test.ranges()[i] },
        ]
    });
}
