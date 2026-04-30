use crate::{SemanticModelOptions, semantic_model};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::JsFileSource;

fn assert_format_snapshot(test_name: &str, source: &str, source_type: JsFileSource) {
    let parsed = biome_js_parser::parse(source, source_type, JsParserOptions::default());
    let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
    let formatted = model.to_string();

    let mut content = String::new();
    content.push_str("## Source\n\n```\n");
    content.push_str(source.trim());
    content.push_str("\n```\n\n## Semantic Model\n\n```\n");
    content.push_str(formatted.trim());
    content.push_str("\n```\n");

    insta::with_settings!({
        snapshot_path => "../snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(test_name, content);
    });
}

#[test]
fn format_js() {
    assert_format_snapshot(
        "format_js",
        r#"
import { foo } from "bar";

const { a, b: renamed } = foo;

let count = 0;

/**
 * Adds two numbers.
 * @param x - first operand
 * @param y - second operand
 */
function add(x, y) {
    return x + y;
}

/** A simple counter. */
class Counter {
    constructor(initial) {
        this.value = initial;
    }
    increment() {
        this.value++;
    }
}

function outer() {
    let local = 1;
    return () => {
        count++;
        return local + count;
    };
}

for (let i = 0; i < 10; i++) {
    console.log(i);
}

try {
    add(a, renamed);
} catch (e) {
    console.log(e);
}

export default outer;
export { add, Counter };
"#,
        JsFileSource::js_module(),
    );
}

#[test]
fn format_ts() {
    assert_format_snapshot(
        "format_ts",
        r#"
import { helper } from "./utils";

/** The config shape. */
interface Config {
    name: string;
    value: number;
}

type Pair<T> = { first: T; second: T };

enum Status {
    Active,
    Inactive,
}

/**
 * Creates a greeting.
 * @param cfg - the configuration
 */
export function greet(cfg: Config): string {
    return `Hello ${cfg.name}`;
}

/** The default config. */
export const DEFAULT: Config = { name: "world", value: helper(0) };

const s: Status = Status.Active;
const p: Pair<number> = { first: 1, second: 2 };

export { Status };
"#,
        JsFileSource::tsx(),
    );
}
