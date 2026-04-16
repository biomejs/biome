// should generate diagnostics

import "./level2.css";
import { Level1 } from "./multilevel.jsx";

export function Level2() {
	return <div className="level2 undefined-at-level2"><Level1 /></div>;
}
