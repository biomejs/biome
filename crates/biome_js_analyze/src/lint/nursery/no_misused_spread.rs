use crate::services::typed::Typed;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsObjectMemberList, JsSpread, JsxSpreadAttribute};
use biome_js_type_info::ObjectSpreadMisuse;
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_misused_spread::NoMisusedSpreadOptions;

declare_lint_rule! {
    /// Disallow spreading values in ways that can cause unexpected results.
    ///
    /// The spread syntax (`...`) copies values into a new array, object, or
    /// list of arguments. For some values the copy doesn't contain what you
    /// might expect:
    ///
    /// - Spreading a string into an array, as in `[...text]`, splits it into
    ///   Unicode code points. Characters made of several code points, such as
    ///   many emoji, end up split into separate pieces.
    /// - Spreading a `Promise` into an object copies nothing. The value it
    ///   resolves to is only available after awaiting it.
    /// - Spreading a function into an object copies nothing, unless the
    ///   function has properties of its own.
    /// - Spreading a `Map` into an object creates an empty object, because the
    ///   entries of a map aren't properties.
    /// - Spreading an array into an object creates properties named after its
    ///   indexes, such as `0` and `1`.
    /// - Spreading another iterable value, such as a `Set`, into an object
    ///   doesn't copy the values it contains.
    /// - Spreading a class instance into an object creates a plain object
    ///   without the methods and getters of the class.
    /// - Spreading a class into an object only copies its static properties.
    ///
    /// This rule uses type information to find these values.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-string.ts
    /// declare const text: string;
    /// const characters = [...text];
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-promise.ts
    /// declare function getUser(): Promise<{ name: string }>;
    /// const user = { ...getUser() };
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-map.ts
    /// declare const scores: Map<string, number>;
    /// const copy = { ...scores };
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-class-instance.ts
    /// class Point {
    ///     constructor(public x: number, public y: number) {}
    ///     length() {
    ///         return Math.hypot(this.x, this.y);
    ///     }
    /// }
    /// const copy = { ...new Point(1, 2) };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// declare const text: string;
    /// const words = text.split(" ");
    ///
    /// declare const items: number[];
    /// const copy = [...items];
    ///
    /// declare function getUser(): Promise<{ name: string }>;
    /// async function loadUser() {
    ///     return { ...(await getUser()) };
    /// }
    ///
    /// declare const scores: Map<string, number>;
    /// const entries = Object.fromEntries(scores);
    /// ```
    pub NoMisusedSpread {
        version: "next",
        name: "noMisusedSpread",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("no-misused-spread").same()],
        recommended: false,
        domains: &[RuleDomain::Types],
    }
}

declare_node_union! {
    pub AnyJsSpreadLike = JsSpread | JsxSpreadAttribute
}

impl Rule for NoMisusedSpread {
    type Query = Typed<AnyJsSpreadLike>;
    type State = SpreadMisuse;
    type Signals = Option<Self::State>;
    type Options = NoMisusedSpreadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let ty = ctx.type_of_expression(&node.argument()?)?;
        if node.is_object_spread() {
            ty.object_spread_misuse().map(SpreadMisuse::Object)
        } else {
            ty.has_string_like_variant().then_some(SpreadMisuse::String)
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().argument()?.range();
        let diagnostic = match state {
            SpreadMisuse::String => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Spreading a string splits it into code points, which can break characters apart."
                },
            )
            .note(markup! {
                "Characters made of several code points, such as many emoji and some letters with accents, are split into separate pieces."
            })
            .note(markup! {
                "Use "<Emphasis>"Intl.Segmenter"</Emphasis>" to split the string into the characters that a reader sees. If the string only contains simple characters, suppress this diagnostic."
            }),
            SpreadMisuse::Object(ObjectSpreadMisuse::Promise) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Spreading a "<Emphasis>"Promise"</Emphasis>" into an object copies nothing."
                },
            )
            .note(markup! {
                "The value of a "<Emphasis>"Promise"</Emphasis>" is only available after it resolves."
            })
            .note(markup! {
                "Did you forget to "<Emphasis>"await"</Emphasis>" it?"
            }),
            SpreadMisuse::Object(ObjectSpreadMisuse::Function) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Spreading a function into an object copies nothing."
                },
            )
            .note(markup! {
                "Only the properties of the function are copied, and this function has none."
            })
            .note(markup! {
                "Did you forget to call the function?"
            }),
            SpreadMisuse::Object(ObjectSpreadMisuse::Map) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Spreading a map into an object creates an empty object."
                },
            )
            .note(markup! {
                "The entries of a map aren't properties, so they aren't copied."
            })
            .note(markup! {
                "To turn the entries of a "<Emphasis>"Map"</Emphasis>" into properties, use "<Emphasis>"Object.fromEntries()"</Emphasis>"."
            }),
            SpreadMisuse::Object(ObjectSpreadMisuse::Array) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Spreading an array into an object creates properties named after its indexes."
                },
            )
            .note(markup! {
                "Each element becomes a property such as "<Emphasis>"0"</Emphasis>" or "<Emphasis>"1"</Emphasis>"."
            })
            .note(markup! {
                "To copy the elements, spread the array into another array instead."
            }),
            SpreadMisuse::Object(ObjectSpreadMisuse::Iterable) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Spreading an iterable value into an object doesn't copy the values it contains."
                },
            )
            .note(markup! {
                "Values such as the items of a "<Emphasis>"Set"</Emphasis>" are produced by iterating, and aren't properties."
            })
            .note(markup! {
                "To copy the values, spread them into an array instead."
            }),
            SpreadMisuse::Object(ObjectSpreadMisuse::ClassInstance) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Spreading a class instance into an object loses the methods of its class."
                },
            )
            .note(markup! {
                "Only the own properties of the instance are copied into a plain object. Methods and getters of the class aren't."
            })
            .note(markup! {
                "Keep using the instance, or create a new one with the class."
            }),
            SpreadMisuse::Object(ObjectSpreadMisuse::Class) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Spreading a class into an object only copies its static properties."
                },
            )
            .note(markup! {
                "The new object is a plain object: it can't create instances and has none of the methods of the class."
            })
            .note(markup! {
                "Did you mean to spread an instance of the class?"
            }),
        };
        Some(diagnostic)
    }
}

/// Describes why a spread is likely a mistake.
#[derive(Clone, Copy)]
pub enum SpreadMisuse {
    /// A string spread into an array or a list of arguments.
    String,
    /// A value spread into an object or into JSX attributes.
    Object(ObjectSpreadMisuse),
}

impl AnyJsSpreadLike {
    fn argument(&self) -> Option<AnyJsExpression> {
        match self {
            Self::JsSpread(spread) => spread.argument().ok(),
            Self::JsxSpreadAttribute(attribute) => attribute.argument().ok(),
        }
    }

    /// Returns whether the spread copies properties into an object or into
    /// JSX attributes, rather than elements into an array or arguments.
    fn is_object_spread(&self) -> bool {
        match self {
            Self::JsSpread(spread) => spread
                .syntax()
                .parent()
                .is_some_and(|parent| JsObjectMemberList::can_cast(parent.kind())),
            Self::JsxSpreadAttribute(_) => true,
        }
    }
}
