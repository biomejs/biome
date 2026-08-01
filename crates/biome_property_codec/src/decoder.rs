use crate::data::{PropertyFmtContext, PropertySyntax};

/// Formats a parsed property syntax as a normalized syntax string.
pub fn decode(value: &PropertySyntax) -> String {
    let formatted = biome_formatter::format!(PropertyFmtContext::default(), [&value])
        .expect("property syntax formatting should not fail");
    formatted
        .print()
        .expect("property syntax should produce a valid document")
        .into_code()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PropertySyntaxResult, encode};
    use biome_rowan::TextRange;

    fn round_trip(source: &str) -> String {
        let syntax = match encode(
            source,
            TextRange::new(0.into(), (source.len() as u32).into()),
        ) {
            PropertySyntaxResult::Value(value) => value,
            result => panic!("expected value, got {result:?}"),
        };
        decode(&syntax)
    }

    #[test]
    fn decodes_canonical_spacing() {
        assert_eq!(
            round_trip("  <length>|auto  |  <color># "),
            "<length> | auto | <color>#"
        );
    }

    #[test]
    fn decodes_universal_syntax() {
        assert_eq!(round_trip(" * "), "*");
    }

    #[test]
    fn decodes_escaped_identifiers() {
        assert_eq!(round_trip("\\66 oo | \\+"), "foo | \\+");
        assert_eq!(round_trip("\\31 foo | \\1f914"), "\\31 foo | 🤔");
        assert_eq!(round_trip("\\a7"), "\\a7 ");
        assert_eq!(round_trip("\\-"), "\\-");
    }

    #[test]
    fn encoding_after_decoding_preserves_the_model() {
        let source = "foo | <color># | <length>+";
        let first = match encode(
            source,
            TextRange::new(0.into(), (source.len() as u32).into()),
        ) {
            PropertySyntaxResult::Value(value) => value,
            result => panic!("expected value, got {result:?}"),
        };
        let decoded = decode(&first);
        let second = match encode(
            &decoded,
            TextRange::new(0.into(), (decoded.len() as u32).into()),
        ) {
            PropertySyntaxResult::Value(value) => value,
            result => panic!("expected value, got {result:?}"),
        };

        assert_eq!(first, second);
    }
}
