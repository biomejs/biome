/* should not generate diagnostics */

import { Block } from "./Block.jsx";
import { Button } from "./Button.jsx"

export function Page() {
	return <div className="page">
		<Button />
		<Block />
	</div>;
}
