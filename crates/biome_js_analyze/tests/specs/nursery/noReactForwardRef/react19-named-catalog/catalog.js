/* should generate diagnostics with pnpm named catalog */

import { forwardRef } from "react";

const Component = forwardRef((props, ref) => {
	return <div ref={ref} {...props} />;
});
