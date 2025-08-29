// negation patterns with logical OR

// basic negation cases
!foo || !foo.bar
!foo || !foo.bar.baz
!foo || !foo()
!foo.bar || !foo.bar.baz
!foo || !foo.bar || !foo.bar.baz
!foo.bar || !foo.bar.baz || !foo.bar.baz.buzz

// with element access
!foo || !foo[bar]
!foo || !foo[bar].baz
!foo[bar] || !foo[bar].baz

// with function calls
!foo || !foo.bar()
!foo.bar || !foo.bar()
!foo || !foo.bar || !foo.bar.baz()

// complex cases
!foo || !foo.bar || !foo.bar.baz || !foo.bar.baz.buzz
!foo || !foo[bar] || !foo[bar].baz || !foo[bar].baz.buzz

// should also handle parentheses
!(foo) || !(foo.bar)
!(foo) || !(foo).bar

// mixed with other expressions (should still be handled)
!foo || !foo.bar || someOtherCondition
someCondition || !foo || !foo.bar

// cases that should NOT be transformed
!foo || !bar
!foo.bar || !baz.bar
foo || !foo.bar  // not both negated
!foo || foo.bar  // not both negated