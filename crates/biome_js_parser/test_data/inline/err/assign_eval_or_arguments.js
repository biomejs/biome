eval = 0
eval ??= 2
eval *= 4
arguments = "foo"
arguments ||= "baz"
({ eval } = o)
({ foo: { eval }}) = o
