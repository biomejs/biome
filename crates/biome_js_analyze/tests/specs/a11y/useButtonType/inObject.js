// invalid
React.createElement('button');
React.createElement('button', {
    "type": "bar"
});
React.createElement('button', {
    "type": 1
});
React.createElement('button', {
    "style": "background: red"
});
React.createElement('button', {});

// valid
React.createElement('button', { "type": foo });
React.createElement("button", { type: "button" }, "foo")