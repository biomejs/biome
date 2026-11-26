use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange};
use biome_tailwind_syntax::{
    AnyCssFunction, AnyCssGenericComponentValue, AnyCssValue, AnyTwCandidate, AnyTwFullCandidate,
    AnyTwValue, TwCandidateList, TwRoot,
};
use smallvec::SmallVec;
use std::ops::Range;

/// An arbitrary value that only reads a CSS variable, such as `[var(--brand)]`.
///
/// Tailwind CSS reads `(--brand)` as `[var(--brand)]`, so the two generate the same CSS.
#[derive(Clone, Copy, Debug)]
pub struct ArbitraryVariableValue {
    /// The range of the arbitrary value within the class string, including its brackets.
    pub range: TextRange,
    /// The range of the text between the parentheses of `var()`.
    arguments: TextRange,
}

impl ArbitraryVariableValue {
    /// Returns the class string of `root` with this value written as a variable shorthand.
    ///
    /// The arguments of `var()` are copied as written, so a fallback value is kept.
    pub fn fixed_class_string(&self, root: &TwRoot) -> Option<String> {
        let text = root.syntax().text_with_trivia().to_string();
        let mut fixed = String::with_capacity(text.len());
        fixed.push_str(text.get(..usize::from(self.range.start()))?);
        fixed.push('(');
        fixed.push_str(text.get(Range::<usize>::from(self.arguments))?);
        fixed.push(')');
        fixed.push_str(text.get(usize::from(self.range.end())..)?);
        Some(fixed)
    }

    /// Returns `value` if it holds a single `var()` call and nothing else.
    fn from_value(value: AnyTwValue) -> Option<Self> {
        let AnyTwValue::TwArbitraryValue(value) = value else {
            return None;
        };
        let mut components = value.value().iter();
        let AnyCssGenericComponentValue::AnyCssValue(AnyCssValue::AnyCssFunction(
            AnyCssFunction::CssFunction(function),
        )) = components.next()?
        else {
            return None;
        };
        if components.next().is_some()
            || function.name().ok()?.ident_token().ok()?.text_trimmed() != "var"
        {
            return None;
        }
        let l_paren = function.l_paren_token().ok()?;
        let arguments = TextRange::new(
            l_paren.text_trimmed_range().end(),
            function.r_paren_token().ok()?.text_trimmed_range().start(),
        );
        // Tailwind CSS only accepts the shorthand when the text between its
        // parentheses starts with `--`.
        let variable = function.parameters().first()?.ok()?;
        let variable = variable
            .as_css_list_of_component_values_expression()?
            .css_component_value_list()
            .first()?;
        if variable.as_css_dashed_identifier()?.range().start() != arguments.start() {
            return None;
        }
        Some(Self {
            range: value.range(),
            arguments,
        })
    }
}

/// Returns the arbitrary values in `candidates` that only read a CSS variable.
///
/// Both the value and the modifier of a utility are checked, as in
/// `bg-[var(--brand)]/[var(--opacity)]`. Variants and arbitrary properties,
/// such as `[color:var(--brand)]`, are not.
pub fn arbitrary_variable_values(
    candidates: &TwCandidateList,
) -> SmallVec<[ArbitraryVariableValue; 1]> {
    let mut values = SmallVec::new();
    for candidate in candidates.iter().flatten() {
        let AnyTwFullCandidate::TwFullCandidate(candidate) = candidate else {
            continue;
        };
        let Ok(AnyTwCandidate::TwFunctionalCandidate(candidate)) = candidate.candidate() else {
            continue;
        };
        let modifier = candidate
            .modifier()
            .and_then(|modifier| modifier.as_tw_modifier()?.value().ok());
        values.extend(
            [candidate.value().ok(), modifier]
                .into_iter()
                .flatten()
                .filter_map(ArbitraryVariableValue::from_value),
        );
    }
    values
}

#[cfg(test)]
mod tests {
    use super::arbitrary_variable_values;
    use biome_tailwind_parser::parse_tailwind;

    #[test]
    fn rewrites_variable_values() {
        for (class, expected) in [
            ("bg-[var(--brand)]", "bg-(--brand)"),
            ("bg-[__var(--brand)__]", "bg-(--brand)"),
            ("bg-[var(--Brand)]", "bg-(--Brand)"),
            ("bg-[var(--größe_variable)]", "bg-(--größe_variable)"),
            ("bg-[var(--brand,_#000)]", "bg-(--brand,_#000)"),
            (
                "bg-[var(--brand,var(--secondary))]",
                "bg-(--brand,var(--secondary))",
            ),
            (
                "h-[var(--header,calc(100%_-_1rem))]",
                "h-(--header,calc(100%_-_1rem))",
            ),
            ("bg-[var(--brand)]!", "bg-(--brand)!"),
            ("!bg-[var(--brand)]", "!bg-(--brand)"),
            ("-mt-[var(--gap)]", "-mt-(--gap)"),
            ("md:hover:bg-[var(--brand)]", "md:hover:bg-(--brand)"),
            ("bg-[var(--brand)]/50", "bg-(--brand)/50"),
            ("bg-red-500/[var(--opacity)]", "bg-red-500/(--opacity)"),
            ("bg-(--brand)/[var(--opacity)]", "bg-(--brand)/(--opacity)"),
        ] {
            let parse = parse_tailwind(class);
            assert!(!parse.has_errors(), "{class}: {:?}", parse.diagnostics());
            let root = parse.tree();
            let found = arbitrary_variable_values(&root.candidates());
            assert_eq!(found.len(), 1, "{class}");
            assert_eq!(
                found[0].fixed_class_string(&root).as_deref(),
                Some(expected),
                "{class}"
            );
        }
    }

    #[test]
    fn reports_value_and_modifier_separately() {
        let source = "p-4 bg-[var(--brand)]/[var(--opacity)] flex";
        let parse = parse_tailwind(source);
        let root = parse.tree();
        let found = arbitrary_variable_values(&root.candidates());
        assert_eq!(found.len(), 2);
        assert_eq!(&source[found[0].range], "[var(--brand)]");
        assert_eq!(&source[found[1].range], "[var(--opacity)]");
        assert_eq!(
            found[0].fixed_class_string(&root).as_deref(),
            Some("p-4 bg-(--brand)/[var(--opacity)] flex")
        );
        assert_eq!(
            found[1].fixed_class_string(&root).as_deref(),
            Some("p-4 bg-[var(--brand)]/(--opacity) flex")
        );
    }

    #[test]
    fn ignores_other_values() {
        for class in [
            "bg-(--brand)",
            "bg-(--brand,_#000)",
            "bg-red-500/(--opacity)",
            "bg-red-500",
            "bg-[#333]",
            "bg-[--brand]",
            "bg-[VAR(--brand)]",
            "bg-[var(_--brand)]",
            "bg-[var()]",
            "bg-[var(brand)]",
            "bg-[theme(--brand)]",
            "object-[var(--x)_var(--y)]",
            "w-[calc(var(--width)+1rem)]",
            "text-[length:var(--size)]",
            "grid-cols-[--spacing(24)_1fr]",
            "[color:var(--brand)]",
            "[--brand:var(--primary)]",
            "min-[var(--breakpoint)]:flex",
            "data-[state=open]:bg-(--brand)",
        ] {
            let parse = parse_tailwind(class);
            assert!(
                arbitrary_variable_values(&parse.tree().candidates()).is_empty(),
                "{class}"
            );
        }
    }
}
