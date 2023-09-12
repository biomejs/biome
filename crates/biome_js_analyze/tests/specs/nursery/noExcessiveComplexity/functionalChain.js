function functionalChain(array) {
    return array
        .filter(Boolean)
        .flatMap(items => {              // nesting = 1
            if (items.length > 1) {      // +2
                return items;
            } else {                     // +1
                return [];
            }
        })
        .filterMap(item => (             // nesting = 1
            item > 0 ? 2 * item : null   // +2
        ));
}
