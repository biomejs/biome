import { introspectTailwindConfig } from "./introspect";
import { sortConfigFromSpec } from "./sort-config";

const EXCLUDED_LAYERS = ["defaults", "base"];
const LAYER_ORDER = ["components", "utilities"];

function generateTailwindPreset() {
	const spec = introspectTailwindConfig(
		{},
		{ excludedLayers: EXCLUDED_LAYERS },
	);
	const sortConfig = sortConfigFromSpec(spec, {
		layerOrder: LAYER_ORDER,
	});
	// TODO: codegen Rust code
	// console.log(sortConfig);
}

generateTailwindPreset();
