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
React.createElement('button', {
    "type": foo
});
