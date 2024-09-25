use crate::{
    js_kinds_src::{AstSrc, Field},
    language_kind::LanguageKind,
};
use biome_string_case::Case;
use xtask::Result;

pub fn generate_target_language_constants(
    ast: &AstSrc,
    _language_kind: LanguageKind,
) -> Result<String> {
    let disregarded_slots: Vec<String> = ast
        .nodes
        .iter()
        .flat_map(|node| {
            let node_kind = Case::Constant.convert(node.name.as_str());
            node.fields
                .iter()
                .enumerate()
                .filter_map(|(index, field)| match field {
                    Field::Token { name, optional, .. } => Some((index, name, optional)),
                    Field::Node { .. } => None,
                })
                // TODO: We might want to move this to `js_kinds_src.rs` when we
                //       start supporting other languages with Grit.
                .filter_map(|(index, name, optional)| match (name.as_str(), optional) {
                    ("async", true) => Some(format!("({node_kind}, {index}, OnlyIf(&[\"\"])),")),
                    (";", true) => Some(format!("({node_kind}, {index}, Always),")),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let disregarded_slots = disregarded_slots.join("\n    ");

    let result = format!(
        "use crate::grit_target_language::DisregardedSlotCondition::{{self, *}};
use biome_js_syntax::JsSyntaxKind::{{self, *}};

pub(crate) const DISREGARDED_SNIPPET_SLOTS: &[(JsSyntaxKind, u32, DisregardedSlotCondition)] = &[
    {disregarded_slots}
];
"
    );

    let pretty = xtask::reformat(result)?;

    Ok(pretty)
}
