_.flatMap(this.visibilityHandlers, fn => fn())
    .concat(this.record.resolved_legacy_visrules)
    .filter(Boolean)

Object
    .keys(
    availableLocales({
        test: true
    })
)
    .forEach(locale => {
        // ...
    });