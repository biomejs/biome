({ ... } = a);
({ ...c = "default" } = a);
({ ...{a} } = b);
({ ...rest, other_assignment } = a);
({ ...rest, } = a);
