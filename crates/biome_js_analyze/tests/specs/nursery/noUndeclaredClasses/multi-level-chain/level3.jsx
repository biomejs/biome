// should generate diagnostics

import "./level3.css";
import { Level2 } from "./level2.jsx";

export function Level3() {
	return <div className="level3 undefined-at-level3"><Level2 /></div>;
}
