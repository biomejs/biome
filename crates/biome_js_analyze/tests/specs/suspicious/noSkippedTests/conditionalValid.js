/* should not generate diagnostics */

// 0-arg bare skip inside an `if` block
test('a', () => {
    if (someCondition) {
        test.skip();
    }
});

// 0-arg bare fixme inside an `if` block
test('b', () => {
    if (someCondition) {
        test.fixme();
    }
});

// 1-arg conditional skip (non-string first arg)
test('c', () => {
    test.skip(someCondition);
});

// 2-arg conditional skip (non-string first arg, string reason)
test('d', () => {
    test.skip(someCondition, 'reason');
});

// it variant: 0-arg bare skip inside if
it('e', () => {
    if (cond) {
        it.skip();
    }
});

// fixme conditional with non-string first arg
test('f', () => {
    test.fixme(someCondition);
});
