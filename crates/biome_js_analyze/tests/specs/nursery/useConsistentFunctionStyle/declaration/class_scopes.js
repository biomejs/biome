// should generate diagnostics
const instanceField = () => class { value = this; };
const staticField = () => class { static value = this; };
const privateField = () => class { #value = this; };
const staticBlock = () => class { static { this.value = 0; } };
const inheritedField = () => class extends Base { value = super.value; };
const inheritedStaticBlock = () => class extends Base { static { super.method(); } };
