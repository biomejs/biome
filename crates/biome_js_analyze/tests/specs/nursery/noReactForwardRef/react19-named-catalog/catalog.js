/* should generate diagnostics */

import { forwardRef } from "react";

const Component = forwardRef((props, ref) => {
	return <div ref={ref} {...props} />;
});
