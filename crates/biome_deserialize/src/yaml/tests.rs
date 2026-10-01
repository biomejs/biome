use super::deserialize_from_yaml_str;
use crate::{
    Deserializable, DeserializableType, DeserializableTypes, DeserializableValue,
    DeserializationContext, DeserializationVisitor, MapMembers, TextNumber,
};
use biome_diagnostics::Error;
use biome_rowan::{Text, TextRange, TextSize};
use std::collections::BTreeMap;

/// Deserializes `source`, and asserts that there are no diagnostics.
#[track_caller]
fn deserialize<Output: Deserializable>(source: &str) -> Output {
    let (deserialized, diagnostics) = deserialize_from_yaml_str(source, "").consume();
    assert!(
        diagnostics.is_empty(),
        "Unexpected diagnostics: {diagnostics:#?}"
    );
    deserialized.expect("The value should be deserialized")
}

/// Deserializes `source`, and returns its diagnostics.
#[track_caller]
fn deserialize_diagnostics<Output: Deserializable>(source: &str) -> Vec<Error> {
    let diagnostics = deserialize_from_yaml_str::<Output>(source, "").into_diagnostics();
    assert!(!diagnostics.is_empty(), "Expected diagnostics");
    diagnostics
}

/// The type of the deserialized value.
#[derive(Debug, Eq, PartialEq)]
struct VisitableType(Option<DeserializableType>);

impl Deserializable for VisitableType {
    fn deserialize(
        _ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        _name: &str,
    ) -> Option<Self> {
        Some(Self(value.visitable_type()))
    }
}

#[track_caller]
fn visitable_type(source: &str) -> Option<DeserializableType> {
    deserialize::<VisitableType>(source).0
}

#[test]
fn test_null() {
    for source in [
        "~",
        "null",
        "Null",
        "NULL",
        "",
        "# comment",
        "---",
        "!!null ''",
    ] {
        assert_eq!(
            visitable_type(source),
            Some(DeserializableType::Null),
            "{source:?}"
        );
    }
    let types = deserialize::<BTreeMap<String, VisitableType>>("a:\nb: &anchor\nc: {d}");
    assert_eq!(types["a"].0, Some(DeserializableType::Null));
    assert_eq!(types["b"].0, Some(DeserializableType::Null));
    assert_eq!(types["c"].0, Some(DeserializableType::Map));
    let types = deserialize::<Vec<VisitableType>>("-\n- 1");
    assert_eq!(types[0].0, Some(DeserializableType::Null));
    let types = deserialize::<Vec<BTreeMap<String, VisitableType>>>("[a: , {b}]");
    assert_eq!(types[0]["a"].0, Some(DeserializableType::Null));
    assert_eq!(types[1]["b"].0, Some(DeserializableType::Null));
}

#[test]
fn test_bool() {
    assert!(deserialize::<bool>("true"));
    assert!(deserialize::<bool>("True"));
    assert!(!deserialize::<bool>("FALSE"));
    // YAML 1.1 booleans are strings
    assert_eq!(deserialize::<String>("yes"), "yes");
    deserialize_diagnostics::<bool>("on");
}

#[test]
fn test_integer() {
    assert_eq!(deserialize::<i64>("42"), 42);
    assert_eq!(deserialize::<i64>("-17"), -17);
    assert_eq!(deserialize::<i64>("+12"), 12);
    assert_eq!(deserialize::<u8>("0o17"), 15);
    assert_eq!(deserialize::<u8>("0x1F"), 31);
    assert_eq!(deserialize::<u8>("0xff"), 255);
    deserialize_diagnostics::<u8>("0x100");
    deserialize_diagnostics::<u8>("1.5");
}

#[test]
fn test_float() {
    assert_eq!(deserialize::<f64>("1.5"), 1.5);
    assert_eq!(deserialize::<f64>(".5"), 0.5);
    assert_eq!(deserialize::<f64>("-2."), -2.0);
    assert_eq!(deserialize::<f64>("+1e3"), 1000.0);
    assert_eq!(deserialize::<f64>("1E-2"), 0.01);
    assert_eq!(deserialize::<f64>("3"), 3.0);
    assert_eq!(deserialize::<f64>(".inf"), f64::INFINITY);
    assert_eq!(deserialize::<f64>("-.Inf"), f64::NEG_INFINITY);
    assert!(deserialize::<f64>(".NaN").is_nan());
}

#[test]
fn test_string() {
    for source in [
        "a", "0o8", "0x", "1.2.3", "1e", ".", "1_000", "-a", "'12'", "\"true\"",
    ] {
        assert_eq!(
            visitable_type(source),
            Some(DeserializableType::Str),
            "{source:?}"
        );
    }
    assert_eq!(deserialize::<String>("hello world"), "hello world");
    deserialize_diagnostics::<String>("a: b");
}

#[test]
fn test_plain_scalar_folding() {
    let source = "a: one\n  two  \n\n\n  three   \nb: four   # comment";
    let map = deserialize::<BTreeMap<String, String>>(source);
    assert_eq!(map["a"], "one two\n\nthree");
    assert_eq!(map["b"], "four");
}

#[test]
fn test_single_quoted_scalar() {
    assert_eq!(deserialize::<String>("'it''s'"), "it's");
    assert_eq!(deserialize::<String>("'  a \\n '"), "  a \\n ");
    assert_eq!(
        deserialize::<String>("'one  \n  two\n\n  three'"),
        "one two\nthree"
    );
}

#[test]
fn test_double_quoted_scalar() {
    assert_eq!(
        deserialize::<String>(r#""\t\n\\\"\/\ \x41\u00e9\U0001F600\0\e\N\_\L\P""#),
        "\t\n\\\"/ A\u{e9}\u{1F600}\0\u{1b}\u{85}\u{a0}\u{2028}\u{2029}"
    );
    assert_eq!(deserialize::<String>("\"  a  \""), "  a  ");
    assert_eq!(
        deserialize::<String>("\"one  \n  two\n\n  three \\t\n four\""),
        "one two\nthree \t four"
    );
    // An escaped line break joins the lines, without removing the preceding blanks
    assert_eq!(deserialize::<String>("\"one \\\n   two\""), "one two");
    assert_eq!(deserialize::<String>("\"one\\\n\n  two\""), "one\ntwo");
    assert_eq!(deserialize::<String>("\"one\r\n  two\""), "one two");
}

#[test]
fn test_literal_block_scalar() {
    // Escaped, because of the lines that only contain spaces
    let source = concat!(
        "clip: |\n  one\n\n    two\n  three\n\n",
        "strip: |-\n  one\n\n",
        "keep: |+\n  one\n    \n  \n",
        "other: |\n  # not a comment\n# comment\n",
        "empty: |\n",
        "last: |\n  end",
    );
    let map = deserialize::<BTreeMap<String, String>>(source);
    assert_eq!(map["clip"], "one\n\n  two\nthree\n");
    assert_eq!(map["strip"], "one");
    // The line with more spaces than the indentation is content
    assert_eq!(map["keep"], "one\n  \n\n");
    assert_eq!(map["other"], "# not a comment\n");
    assert_eq!(map["empty"], "");
    assert_eq!(map["last"], "end");

    // https://yaml.org/spec/1.2.2/#example-empty-scalar-chomping
    let source = "strip: >-\n\nclip: >\n\nkeep: |+\n\n";
    let map = deserialize::<BTreeMap<String, String>>(source);
    assert_eq!((map["strip"].as_str(), map["clip"].as_str()), ("", ""));
    assert_eq!(map["keep"], "\n");

    let source = "crlf: |+\r\n  one\r\n  two\r\n\r\nquoted: \"three\r\n  four\"";
    let map = deserialize::<BTreeMap<String, String>>(source);
    assert_eq!(map["crlf"], "one\ntwo\n\n");
    assert_eq!(map["quoted"], "three four");
}

#[test]
fn test_folded_block_scalar() {
    // https://yaml.org/spec/1.2.2/#example-folded-lines
    let source = ">\n\n folded\n line\n\n next\n line\n   * bullet\n\n   * list\n   * lines\n\n last\n line\n\n# Comment\n";
    assert_eq!(
        deserialize::<String>(source),
        "\nfolded line\nnext line\n  * bullet\n\n  * list\n  * lines\n\nlast line\n"
    );
    // https://yaml.org/spec/1.2.2/#example-block-indentation-indicator
    let source =
        "- |\n detected\n- >\n \n  \n  # detected\n- |1\n  explicit\n- >\n \t\n detected\n";
    assert_eq!(
        deserialize::<Vec<String>>(source),
        [
            "detected\n",
            "\n\n# detected\n",
            " explicit\n",
            "\t\ndetected\n"
        ]
    );
}

#[test]
fn test_block_scalar_indentation_indicator() {
    let source = "a:\n  b: |2\n      one\n    two\n  c: >1-\n    three\n";
    let map = deserialize::<BTreeMap<String, BTreeMap<String, String>>>(source);
    assert_eq!(map["a"]["b"], "  one\ntwo\n");
    assert_eq!(map["a"]["c"], " three");
    let source = "- &anchor a: |1\n     one\n";
    let list = deserialize::<Vec<BTreeMap<String, String>>>(source);
    assert_eq!(list[0]["a"], "  one\n");
}

#[test]
fn test_collections() {
    let source = "\
block:
  a: 1
  ? b
  : 2
sequence:
- 3
- 4
flow: {c: 5, 'd': 6, \"e\": 7}
";
    let map = deserialize::<BTreeMap<String, VisitableType>>(source);
    assert_eq!(map["block"].0, Some(DeserializableType::Map));
    assert_eq!(map["sequence"].0, Some(DeserializableType::Array));
    let map = deserialize::<BTreeMap<String, BTreeMap<String, u8>>>(
        "block:\n  a: 1\n  ? b\n  : 2\nflow: {c: 3, 'd': 4, \"e\": 5}",
    );
    assert_eq!(
        map["block"],
        BTreeMap::from([("a".into(), 1), ("b".into(), 2)])
    );
    assert_eq!(
        map["flow"],
        BTreeMap::from([("c".into(), 3), ("d".into(), 4), ("e".into(), 5)])
    );
    assert_eq!(deserialize::<Vec<u8>>("- 1\n- 2"), [1, 2]);
    assert_eq!(
        deserialize::<Vec<Vec<u8>>>("[[1], [2, 3], []]"),
        [vec![1], vec![2, 3], vec![]]
    );
    // A pair in a flow sequence is a mapping of a single entry
    assert_eq!(
        deserialize::<Vec<BTreeMap<String, u8>>>("[a: 1, {b: 2}]"),
        [
            BTreeMap::from([("a".into(), 1)]),
            BTreeMap::from([("b".into(), 2)])
        ]
    );
}

#[test]
fn test_keys_are_strings() {
    let map = deserialize::<BTreeMap<String, u8>>("1: 1\ntrue: 2\nnull: 3\n'4': 4\n!!int 5: 5");
    assert_eq!(
        map.keys().collect::<Vec<_>>(),
        ["1", "4", "5", "null", "true"]
    );
}

#[test]
fn test_aliases() {
    let source = "\
a: &number 1
b: *number
base: &base
  c: 2
other: *base
seq: &seq [3]
copy: *seq
";
    let map = deserialize::<BTreeMap<String, VisitableType>>(source);
    assert_eq!(map["b"].0, Some(DeserializableType::Number));
    assert_eq!(map["other"].0, Some(DeserializableType::Map));
    assert_eq!(map["copy"].0, Some(DeserializableType::Array));
    let map = deserialize::<BTreeMap<String, BTreeMap<String, u8>>>(
        "base: &base\n  &key a: 1\nother: *base\nkey: {*key : 2}",
    );
    assert_eq!(map["other"]["a"], 1);
    assert_eq!(map["key"]["a"], 2);
    // An alias refers to the last anchor that precedes it
    let source = "a: &x 1\nb: *x\nc: &x 2\nd: *x";
    let map = deserialize::<BTreeMap<String, u8>>(source);
    assert_eq!((map["b"], map["d"]), (1, 2));
}

#[test]
fn test_alias_diagnostics() {
    let diagnostics = deserialize_diagnostics::<BTreeMap<String, u8>>("a: *x\nb: &x 1");
    assert_eq!(diagnostics.len(), 1);
    let diagnostics = deserialize_diagnostics::<Vec<Vec<u8>>>("- &x [*x]");
    assert_eq!(diagnostics.len(), 1);
    // A type mismatch is reported at the anchored node, and at its alias
    let diagnostics = deserialize_diagnostics::<BTreeMap<String, String>>("c: &y 2\nd: *y");
    let spans = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.location().span)
        .collect::<Vec<_>>();
    assert_eq!(
        spans,
        [
            Some(TextRange::new(TextSize::from(3), TextSize::from(7))),
            Some(TextRange::new(TextSize::from(11), TextSize::from(13)))
        ]
    );
}

/// The number of scalars of the deserialized value, which visits all its nodes.
struct ScalarCount(usize);

impl Deserializable for ScalarCount {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, ScalarCountVisitor, name)
    }
}

struct ScalarCountVisitor;

impl DeserializationVisitor for ScalarCountVisitor {
    type Output = ScalarCount;
    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::all();

    fn visit_null(
        self,
        _ctx: &mut dyn DeserializationContext,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        Some(ScalarCount(1))
    }

    fn visit_bool(
        self,
        _ctx: &mut dyn DeserializationContext,
        _value: bool,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        Some(ScalarCount(1))
    }

    fn visit_number(
        self,
        _ctx: &mut dyn DeserializationContext,
        _value: TextNumber,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        Some(ScalarCount(1))
    }

    fn visit_str(
        self,
        _ctx: &mut dyn DeserializationContext,
        _value: Text,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        Some(ScalarCount(1))
    }

    fn visit_array(
        self,
        ctx: &mut dyn DeserializationContext,
        items: &mut dyn ExactSizeIterator<Item = Option<Box<dyn DeserializableValue>>>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let count = items
            .flatten()
            .filter_map(|item| ScalarCount::deserialize(ctx, &item, ""))
            .map(|count| count.0)
            .sum();
        Some(ScalarCount(count))
    }

    fn visit_map(
        self,
        ctx: &mut dyn DeserializationContext,
        members: &mut MapMembers<'_>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let count = members
            .flatten()
            .filter_map(|(_, value)| ScalarCount::deserialize(ctx, &value, ""))
            .map(|count| count.0)
            .sum();
        Some(ScalarCount(count))
    }
}

#[test]
fn test_alias_expansion_limit() {
    // Each level has 10 aliases to the previous one
    let mut source = String::from("l0: &l0 [x, x, x, x, x, x, x, x, x, x]\n");
    for level in 1..10 {
        let previous = level - 1;
        let aliases = vec![format!("*l{previous}"); 10].join(", ");
        source.push_str(&format!("l{level}: &l{level} [{aliases}]\n"));
    }
    let count = deserialize::<ScalarCount>(&source[..source.find("l3").unwrap()]);
    assert_eq!(count.0, 10 + 100 + 1000);
    // `l9` alone would expand to a billion scalars
    let diagnostics = deserialize_diagnostics::<ScalarCount>(&source);
    assert_eq!(diagnostics.len(), 1, "{diagnostics:#?}");
}

#[test]
fn test_tags() {
    assert_eq!(deserialize::<String>("!!str 12"), "12");
    assert_eq!(deserialize::<String>("! true"), "true");
    assert_eq!(deserialize::<String>("!<tag:yaml.org,2002:str> ~"), "~");
    assert_eq!(deserialize::<u8>("!!int '12'"), 12);
    assert_eq!(deserialize::<f64>("!!float \"1\""), 1.0);
    assert!(deserialize::<bool>("!!bool 'true'"));
    // Other tags don't change the resolution
    assert_eq!(deserialize::<u8>("!custom 12"), 12);
    deserialize_diagnostics::<u8>("!!int abc");
    assert_eq!(visitable_type("!!int abc"), None);
}

#[test]
fn test_documents() {
    assert_eq!(
        deserialize::<String>("%YAML 1.2\n--- first\n...\n--- second"),
        "first"
    );
    assert_eq!(deserialize::<String>("--- |\n  1"), "1");
}
