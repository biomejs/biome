use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsCallArgument, AnyJsClass, AnyJsClassMember,
    AnyJsClassMemberName, AnyJsConstructorParameter, AnyJsExpression, AnyJsFormalParameter,
    AnyJsObjectMember, JsAssignmentExpression, JsAssignmentOperator, JsCallArgumentList,
    JsCallExpression, JsConstructorClassMember, JsPropertyClassMember, JsSyntaxToken,
    AnyJsGetter, AnyJsSetter, global_identifier,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxResult, TextRange, TokenText};
use biome_rule_options::use_custom_error_definition::UseCustomErrorDefinitionOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce a consistent way of defining custom error classes.
    ///
    /// A custom error class should have a name ending with `Error`, set its `name` property to
    /// that name, and pass the message to `super()`. This way, it behaves like the built-in errors
    /// and is easy to recognize in stack traces and logs.
    ///
    /// When the class extends a built-in error such as `Error` or `TypeError`, its constructor
    /// should also accept an `options` parameter and pass it to `super()`. The options carry the
    /// `cause` of the error, which keeps the original failure available for debugging.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// // The `name` property isn't set, so the error shows up as `Error`.
    /// class FooError extends Error {
    ///     constructor(message, options) {
    ///         super(message, options);
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class FooError extends Error {
    ///     constructor(message, options) {
    ///         super(message, options);
    ///         // Tools that shrink code can rename the class. Use a string instead.
    ///         this.name = this.constructor.name;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class FooError extends Error {
    ///     constructor(message, options) {
    ///         // Pass the message to `super()` instead of setting `this.message`.
    ///         super(undefined, options);
    ///         this.message = message;
    ///         this.name = "FooError";
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // The class name should end with `Error`.
    /// class Foo extends Error {
    ///     name = "Foo";
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class FooError extends Error {
    ///     // The constructor should accept an `options` parameter and pass it to `super()`.
    ///     constructor(message) {
    ///         super(message);
    ///         this.name = "FooError";
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class FooError extends Error {
    ///     constructor(message, options) {
    ///         // The message passed to `super()` hides the `message` getter below.
    ///         super(message, options);
    ///         this.name = "FooError";
    ///     }
    ///
    ///     get message() {
    ///         return "Custom message";
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class FooError extends Error {
    ///     constructor(message, options) {
    ///         super(message, options);
    ///         this.name = "FooError";
    ///     }
    /// }
    ///
    /// class ApiError extends Error {
    ///     constructor(status, message, options) {
    ///         super(message, options);
    ///         this.name = "ApiError";
    ///         this.status = status;
    ///     }
    /// }
    ///
    /// class TimeoutError extends Error {
    ///     constructor(cause) {
    ///         super("The request timed out", { cause });
    ///         this.name = "TimeoutError";
    ///     }
    /// }
    ///
    /// class BarError extends TypeError {
    ///     name = "BarError";
    /// }
    ///
    /// class MessageError extends Error {
    ///     #message;
    ///
    ///     constructor(message, options) {
    ///         super(undefined, options);
    ///         this.#message = message;
    ///         this.name = "MessageError";
    ///     }
    ///
    ///     get message() {
    ///         return this.#message;
    ///     }
    /// }
    /// ```
    ///
    pub UseCustomErrorDefinition {
        version: "next",
        name: "useCustomErrorDefinition",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("custom-error-definition").inspired()],
    }
}

/// A problem found in the definition of a custom error class.
#[derive(Debug, Clone, Copy)]
pub enum CustomErrorProblem {
    /// The class name doesn't start with an uppercase letter or doesn't end with `Error`.
    InvalidClassName,
    /// The `name` property isn't set to the class name. The range covers the assigned value, or
    /// the class name when nothing is assigned.
    InvalidName(TextRange),
    /// The constructor doesn't call `super()`. The range covers the `constructor` keyword.
    MissingSuperCall(TextRange),
    /// `super()` receives a message although the class defines a `message` accessor.
    MessagePassedWithAccessor(TextRange),
    /// `this.message` is assigned although the class defines a `message` getter without a setter.
    MessageAssignedWithoutSetter(TextRange),
    /// `this.message` is assigned instead of passing the message to `super()`.
    MessageAssigned(TextRange),
    /// The constructor has a single parameter and none named `options`. The range covers that
    /// parameter.
    MissingOptionsParameter(TextRange),
    /// The constructor has several parameters and none of them is a non-rest parameter named
    /// `options`. The range covers the `constructor` keyword.
    InvalidOptionsParameter(TextRange),
    /// The `message` parameter isn't the first argument of `super()`.
    MissingMessageArgument(TextRange),
    /// The `options` parameter isn't the second argument of `super()`.
    MissingOptionsArgument(TextRange),
    /// The class is assigned to an `exports` property with a different name. The range covers
    /// the property name.
    InvalidExportName(TextRange),
}

impl Rule for UseCustomErrorDefinition {
    type Query = Semantic<AnyJsClass>;
    type State = CustomErrorProblem;
    type Signals = Box<[Self::State]>;
    type Options = UseCustomErrorDefinitionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        // Classes in a declaration file only describe types and never run.
        if ctx
            .source_type::<JsFileSource>()
            .language()
            .is_definition_file()
        {
            return Box::default();
        }
        let mut problems = Vec::new();
        check_class(ctx.query(), ctx.model(), &mut problems);
        problems.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, problem: &Self::State) -> Option<RuleDiagnostic> {
        let id = ctx.query().id()?;
        let name_token = id.as_js_identifier_binding()?.name_token().ok()?;
        let name = name_token.text_trimmed();
        Some(match *problem {
            CustomErrorProblem::InvalidClassName => RuleDiagnostic::new(
                rule_category!(),
                name_token.text_trimmed_range(),
                markup! {
                    "The name of the error class "<Emphasis>{name}</Emphasis>" doesn't start with an uppercase letter and end with "<Emphasis>"Error"</Emphasis>"."
                },
            )
            .note(markup! {
                "A consistent naming convention makes error classes easy to recognize."
            })
            .note(markup! {
                "Rename the class to "<Emphasis>{ExpectedClassName(name)}</Emphasis>"."
            }),
            CustomErrorProblem::InvalidName(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>"name"</Emphasis>" property of this error class isn't set to "<Emphasis>"\""{name}"\""</Emphasis>"."
                },
            )
            .note(markup! {
                "The "<Emphasis>"name"</Emphasis>" property is shown in stack traces and logs. When it isn't the class name, the error is hard to tell apart from other errors."
            })
            .note(markup! {
                "Set the "<Emphasis>"name"</Emphasis>" property to the string "<Emphasis>"\""{name}"\""</Emphasis>", either in the class body or in the constructor. Use a string rather than "<Emphasis>"this.constructor.name"</Emphasis>", because tools that shrink code can rename the class."
            }),
            CustomErrorProblem::MissingSuperCall(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This error constructor doesn't call "<Emphasis>"super()"</Emphasis>"."
                },
            )
            .note(markup! {
                "A class that extends another class must call "<Emphasis>"super()"</Emphasis>" in its constructor. Otherwise, creating the error throws a "<Emphasis>"ReferenceError"</Emphasis>"."
            })
            .note(markup! {
                "Call "<Emphasis>"super()"</Emphasis>" in the constructor, and pass it the error message."
            }),
            CustomErrorProblem::MessagePassedWithAccessor(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The error message is passed to "<Emphasis>"super()"</Emphasis>", but the class defines its own "<Emphasis>"message"</Emphasis>" accessor."
                },
            )
            .note(markup! {
                "The message passed to "<Emphasis>"super()"</Emphasis>" is stored directly on the error, which hides the "<Emphasis>"message"</Emphasis>" getter and setter of the class."
            })
            .note(markup! {
                "Pass "<Emphasis>"undefined"</Emphasis>" as the first argument of "<Emphasis>"super()"</Emphasis>", and store the message somewhere else, such as in a private field."
            }),
            CustomErrorProblem::MessageAssignedWithoutSetter(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This code assigns "<Emphasis>"this.message"</Emphasis>", but the class only defines a getter for "<Emphasis>"message"</Emphasis>"."
                },
            )
            .note(markup! {
                "A property with a getter and no setter can't be assigned, so creating the error throws a "<Emphasis>"TypeError"</Emphasis>"."
            })
            .note(markup! {
                "Remove the assignment, or add a "<Emphasis>"message"</Emphasis>" setter to the class."
            }),
            CustomErrorProblem::MessageAssigned(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The error message is set with "<Emphasis>"this.message"</Emphasis>" instead of being passed to "<Emphasis>"super()"</Emphasis>"."
                },
            )
            .note(markup! {
                "The "<Emphasis>"Error"</Emphasis>" constructor already sets the message from its first argument."
            })
            .note(markup! {
                "Pass the message to "<Emphasis>"super()"</Emphasis>" as the first argument, and remove this assignment."
            }),
            CustomErrorProblem::MissingOptionsParameter(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This error constructor doesn't accept an "<Emphasis>"options"</Emphasis>" parameter."
                },
            )
            .note(markup! {
                "The options object lets the code that creates the error attach the original error as its "<Emphasis>"cause"</Emphasis>". This keeps the chain of failures available for debugging."
            })
            .note(markup! {
                "Add an "<Emphasis>"options"</Emphasis>" parameter, and pass it to "<Emphasis>"super()"</Emphasis>" as the second argument."
            }),
            CustomErrorProblem::InvalidOptionsParameter(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This error constructor doesn't accept the error options in a single parameter named "<Emphasis>"options"</Emphasis>"."
                },
            )
            .note(markup! {
                "The options object lets the code that creates the error attach the original error as its "<Emphasis>"cause"</Emphasis>". This keeps the chain of failures available for debugging."
            })
            .note(markup! {
                "Accept the options in a parameter named "<Emphasis>"options"</Emphasis>" that isn't a rest parameter such as "<Emphasis>"...options"</Emphasis>", and pass it to "<Emphasis>"super()"</Emphasis>" as the second argument. Other values can be read from it, such as "<Emphasis>"options.details"</Emphasis>"."
            }),
            CustomErrorProblem::MissingMessageArgument(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>"message"</Emphasis>" parameter isn't passed to "<Emphasis>"super()"</Emphasis>"."
                },
            )
            .note(markup! {
                "The "<Emphasis>"Error"</Emphasis>" constructor sets the message from its first argument, so the error doesn't get the message it was created with."
            })
            .note(markup! {
                "Pass "<Emphasis>"message"</Emphasis>" to "<Emphasis>"super()"</Emphasis>" as the first argument."
            }),
            CustomErrorProblem::MissingOptionsArgument(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The "<Emphasis>"options"</Emphasis>" parameter isn't passed to "<Emphasis>"super()"</Emphasis>"."
                },
            )
            .note(markup! {
                "The "<Emphasis>"Error"</Emphasis>" constructor reads options such as "<Emphasis>"cause"</Emphasis>" from its second argument, so the options the error was created with are lost."
            })
            .note(markup! {
                "Pass "<Emphasis>"options"</Emphasis>" to "<Emphasis>"super()"</Emphasis>" as the second argument."
            }),
            CustomErrorProblem::InvalidExportName(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This export name doesn't match the name of the error class "<Emphasis>{name}</Emphasis>"."
                },
            )
            .note(markup! {
                "Exporting an error class under its own name makes it easy to find where the error is defined."
            })
            .note(markup! {
                "Rename the export to "<Emphasis>{name}</Emphasis>"."
            }),
        })
    }
}

/// Pushes every problem found in `class` to `problems`.
///
/// Classes without a name, abstract and ambient classes, and classes that don't extend an error
/// class are skipped.
fn check_class(
    class: &AnyJsClass,
    model: &SemanticModel,
    problems: &mut Vec<CustomErrorProblem>,
) -> Option<()> {
    if class.abstract_token().is_some()
        || class
            .as_js_class_declaration()
            .is_some_and(|declaration| declaration.is_ambient())
    {
        return None;
    }
    let name_token = class.id()?.as_js_identifier_binding()?.name_token().ok()?;
    let name = name_token.text_trimmed();
    let super_class = class
        .extends_clause()?
        .super_class()
        .ok()?
        .omit_type_wrappers()?;
    if !super_class_name(&super_class).is_some_and(|name| is_error_class_name(name.text())) {
        return None;
    }

    let has_valid_class_name = is_valid_class_name(name);
    if !has_valid_class_name {
        problems.push(CustomErrorProblem::InvalidClassName);
    }
    if let Some(range) = invalid_export_name_range(class, name, model) {
        problems.push(CustomErrorProblem::InvalidExportName(range));
    }

    let mut constructor = None;
    let mut has_constructor_signature = false;
    let mut name_property = None;
    let mut has_message_getter = false;
    let mut has_message_setter = false;
    for member in class.members() {
        match member {
            AnyJsClassMember::JsConstructorClassMember(member) => {
                constructor.get_or_insert(member);
            }
            AnyJsClassMember::TsConstructorSignatureClassMember(_) => {
                has_constructor_signature = true;
            }
            AnyJsClassMember::JsPropertyClassMember(member) => {
                if name_property.is_none()
                    && !member
                        .modifiers()
                        .iter()
                        .any(|modifier| modifier.as_js_static_modifier().is_some())
                    && member
                        .name()
                        .is_ok_and(|member_name| is_literal_member_named(&member_name, "name"))
                {
                    name_property = Some(member);
                }
            }
            AnyJsClassMember::JsGetterClassMember(member) => {
                let getter = AnyJsGetter::from(member);
                has_message_getter |= !getter.is_static()
                    && getter.member_name().is_some_and(|name| name == "message");
            }
            AnyJsClassMember::JsSetterClassMember(member) => {
                let setter = AnyJsSetter::from(member);
                has_message_setter |= !setter.is_static()
                    && setter.member_name().is_some_and(|name| name == "message");
            }
            _ => {}
        }
    }

    let Some(constructor) = constructor else {
        // A constructor declared without a body, such as an overload, can't be checked.
        if !has_constructor_signature
            && let Some(range) = invalid_name_property_range(name_property.as_ref(), &name_token)
        {
            problems.push(CustomErrorProblem::InvalidName(range));
        }
        return None;
    };

    let mut super_call = None;
    let mut message_assignment = None;
    let mut name_value = None;
    for statement in constructor.body().ok()?.statements() {
        let Some(expression) = statement
            .as_js_expression_statement()
            .and_then(|statement| statement.expression().ok())
        else {
            continue;
        };
        match expression {
            AnyJsExpression::JsCallExpression(call) => {
                if super_call.is_none()
                    && call
                        .callee()
                        .is_ok_and(|callee| callee.as_js_super_expression().is_some())
                {
                    super_call = Some(call);
                }
            }
            AnyJsExpression::JsAssignmentExpression(assignment) => {
                match this_member_assignment_name(&assignment).as_ref().map(TokenText::text) {
                    Some("message") => {
                        message_assignment.get_or_insert(assignment.range());
                    }
                    Some("name") if name_value.is_none() => {
                        name_value = assignment.right().ok();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let has_message_accessor = has_message_getter || has_message_setter;
    let has_body_problem = match (&super_call, message_assignment) {
        (None, _) => {
            let range = constructor.name().ok()?.range();
            problems.push(CustomErrorProblem::MissingSuperCall(range));
            true
        }
        (Some(call), _)
            if has_message_accessor
                && !is_missing_or_undefined(call.arguments().ok()?.args().iter().next()) =>
        {
            problems.push(CustomErrorProblem::MessagePassedWithAccessor(call.range()));
            true
        }
        (Some(_), Some(range)) if has_message_getter && !has_message_setter => {
            problems.push(CustomErrorProblem::MessageAssignedWithoutSetter(range));
            true
        }
        (Some(_), Some(range)) if !has_message_setter => {
            problems.push(CustomErrorProblem::MessageAssigned(range));
            true
        }
        _ => false,
    };

    let invalid_name_range = match name_value {
        Some(value) => (!is_string_value(&value, name)).then(|| value.range()),
        None => invalid_name_property_range(name_property.as_ref(), &name_token),
    };
    if let Some(range) = invalid_name_range {
        problems.push(CustomErrorProblem::InvalidName(range));
        return None;
    }

    if !has_valid_class_name
        || has_body_problem
        || !is_builtin_error_with_options(&super_class, model)
    {
        return None;
    }
    let problem = options_problem(&constructor, super_call.as_ref()?, has_message_accessor)?;
    problems.push(problem);
    None
}

/// Returns the problem with how the constructor receives the error options and passes them to
/// `super()`, if any.
fn options_problem(
    constructor: &JsConstructorClassMember,
    super_call: &JsCallExpression,
    has_message_accessor: bool,
) -> Option<CustomErrorProblem> {
    let parameters = constructor.parameters().ok()?.parameters();
    let first_parameter = parameters.first()?.ok()?;
    let first_name = parameter_name(&first_parameter)?;
    let first_name = first_name.text_trimmed();
    let arguments = super_call.arguments().ok()?.args();

    if first_name == "options" {
        return (!is_identifier_named(arguments.iter().nth(1), "options"))
            .then(|| CustomErrorProblem::MissingOptionsArgument(super_call.range()));
    }

    let has_options_parameter = parameters.iter().any(|parameter| {
        parameter.is_ok_and(|parameter| {
            parameter_name(&parameter).is_some_and(|name| name.text_trimmed() == "options")
        })
    });
    let has_inline_options = has_inline_error_options(&arguments, has_options_parameter);
    if has_inline_options && !has_options_parameter {
        return None;
    }
    if parameters.len() < 2 {
        return Some(CustomErrorProblem::MissingOptionsParameter(
            first_parameter.range(),
        ));
    }
    if !has_options_parameter {
        return Some(CustomErrorProblem::InvalidOptionsParameter(
            constructor.name().ok()?.range(),
        ));
    }
    if !has_message_accessor
        && first_name == "message"
        && !is_identifier_named(arguments.iter().next(), "message")
    {
        return Some(CustomErrorProblem::MissingMessageArgument(super_call.range()));
    }
    if has_inline_options || is_identifier_named(arguments.iter().nth(1), "options") {
        return None;
    }
    Some(CustomErrorProblem::MissingOptionsArgument(super_call.range()))
}

/// Returns whether `super()` already receives the error options as an inline object that only
/// has a `cause` property, such as `super("Fixed message", { cause })`.
///
/// When the constructor has an `options` parameter, the cause must be `options.cause`. Otherwise,
/// the message must be a string literal.
fn has_inline_error_options(arguments: &JsCallArgumentList, has_options_parameter: bool) -> bool {
    if arguments.len() != 2 {
        return false;
    }
    let Some(Ok(AnyJsCallArgument::AnyJsExpression(options))) = arguments.iter().nth(1) else {
        return false;
    };
    let Some(AnyJsExpression::JsObjectExpression(options)) = options.omit_type_wrappers() else {
        return false;
    };
    let members = options.members();
    let Some(Ok(member)) = members.first() else {
        return false;
    };
    if members.len() != 1 || member.name().is_none_or(|name| name.text() != "cause") {
        return false;
    }
    if has_options_parameter {
        let AnyJsObjectMember::JsPropertyObjectMember(member) = member else {
            return false;
        };
        let Some(AnyJsExpression::JsStaticMemberExpression(cause)) = member
            .value()
            .ok()
            .and_then(AnyJsExpression::omit_type_wrappers)
        else {
            return false;
        };
        return cause
            .member()
            .is_ok_and(|member| {
                member.as_js_name().is_some_and(|name| {
                    name.value_token()
                        .is_ok_and(|token| token.text_trimmed() == "cause")
                })
            })
            && cause.object().is_ok_and(|object| {
                object
                    .omit_parentheses()
                    .as_js_reference_identifier()
                    .is_some_and(|reference| reference.has_name("options"))
            });
    }
    let Some(Ok(AnyJsCallArgument::AnyJsExpression(message))) = arguments.iter().next() else {
        return false;
    };
    message
        .omit_type_wrappers()
        .and_then(|message| message.as_static_value())
        .is_some_and(|value| value.as_string_constant().is_some())
}

/// Returns the range to report when the `name` property declared in the class body isn't set to
/// the class name, or `None` when it is.
fn invalid_name_property_range(
    name_property: Option<&JsPropertyClassMember>,
    class_name: &JsSyntaxToken,
) -> Option<TextRange> {
    let Some(name_property) = name_property else {
        return Some(class_name.text_trimmed_range());
    };
    let Some(value) = name_property
        .value()
        .and_then(|initializer| initializer.expression().ok())
    else {
        return Some(name_property.range());
    };
    (!is_string_value(&value, class_name.text_trimmed())).then(|| value.range())
}

/// Returns the range of the property name when `class` is assigned to a property of `exports`
/// whose name differs from `class_name`, as in `exports.foo = class FooError extends Error {}`.
fn invalid_export_name_range(
    class: &AnyJsClass,
    class_name: &str,
    model: &SemanticModel,
) -> Option<TextRange> {
    let class = class.as_js_class_expression()?;
    let assignment = JsAssignmentExpression::cast(class.syntax().parent()?)?;
    let AnyJsAssignmentPattern::AnyJsAssignment(target) = assignment.left().ok()? else {
        return None;
    };
    let (object, has_class_name, range) = match target {
        AnyJsAssignment::JsStaticMemberAssignment(member) => {
            let name = member.member().ok()?;
            let name_token = name.as_js_name()?.value_token().ok()?;
            (
                member.object().ok()?,
                name_token.text_trimmed() == class_name,
                name.range(),
            )
        }
        AnyJsAssignment::JsComputedMemberAssignment(member) => {
            let name = member.member().ok()?.omit_type_wrappers()?;
            let export_name = name.as_static_value()?;
            (
                member.object().ok()?,
                export_name.as_string_constant()? == class_name,
                name.range(),
            )
        }
        _ => return None,
    };
    let reference = object.as_js_reference_identifier()?;
    if has_class_name || !reference.has_name("exports") || model.binding(&reference).is_some() {
        return None;
    }
    Some(range)
}

/// Returns the name of the class `expression` refers to: `Foo` for `Foo` and `Bar` for
/// `foo.Bar`.
fn super_class_name(expression: &AnyJsExpression) -> Option<TokenText> {
    match expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => Some(
            identifier
                .name()
                .ok()?
                .value_token()
                .ok()?
                .token_text_trimmed(),
        ),
        AnyJsExpression::JsStaticMemberExpression(member) => Some(
            member
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()?
                .token_text_trimmed(),
        ),
        _ => None,
    }
}

/// Returns whether `super_class` refers to a global built-in error whose constructor accepts the
/// error options as its second argument.
fn is_builtin_error_with_options(super_class: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some((reference, name)) = super_class
        .as_any_global_identifier_expression()
        .and_then(|expression| global_identifier(&expression))
    else {
        return false;
    };
    model.binding(&reference).is_none()
        && BUILTIN_ERRORS_WITH_OPTIONS
            .binary_search(&name.text())
            .is_ok()
}

/// Returns the name of the `this` property assigned by `assignment` with `=`, such as `message`
/// for `this.message = value`.
fn this_member_assignment_name(assignment: &JsAssignmentExpression) -> Option<TokenText> {
    if assignment.operator().ok()? != JsAssignmentOperator::Assign {
        return None;
    }
    let AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsStaticMemberAssignment(member)) =
        assignment.left().ok()?
    else {
        return None;
    };
    member.object().ok()?.as_js_this_expression()?;
    Some(
        member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .token_text_trimmed(),
    )
}

/// Returns the name token of a constructor parameter that is a plain identifier, possibly with a
/// default value or a TypeScript parameter property modifier.
fn parameter_name(parameter: &AnyJsConstructorParameter) -> Option<JsSyntaxToken> {
    let formal_parameter = match parameter {
        AnyJsConstructorParameter::AnyJsFormalParameter(parameter) => parameter.clone(),
        AnyJsConstructorParameter::TsPropertyParameter(parameter) => {
            parameter.formal_parameter().ok()?
        }
        AnyJsConstructorParameter::JsRestParameter(_) => return None,
    };
    let AnyJsFormalParameter::JsFormalParameter(formal_parameter) = formal_parameter else {
        return None;
    };
    formal_parameter
        .binding()
        .ok()?
        .as_any_js_binding()?
        .as_js_identifier_binding()?
        .name_token()
        .ok()
}

/// Returns whether `argument` is missing or is `undefined`.
fn is_missing_or_undefined(argument: Option<SyntaxResult<AnyJsCallArgument>>) -> bool {
    match argument {
        None => true,
        Some(Ok(AnyJsCallArgument::AnyJsExpression(expression))) => expression
            .omit_type_wrappers()
            .and_then(|expression| expression.as_js_reference_identifier())
            .is_some_and(|reference| reference.is_undefined()),
        Some(_) => false,
    }
}

/// Returns whether `argument` is an identifier named `name`.
fn is_identifier_named(
    argument: Option<SyntaxResult<AnyJsCallArgument>>,
    name: &str,
) -> bool {
    let Some(Ok(AnyJsCallArgument::AnyJsExpression(expression))) = argument else {
        return false;
    };
    expression
        .omit_type_wrappers()
        .and_then(|expression| expression.as_js_reference_identifier())
        .is_some_and(|reference| reference.has_name(name))
}

/// Returns whether `expression` is a string literal, or a template literal without
/// substitutions, whose value is `expected`.
fn is_string_value(expression: &AnyJsExpression, expected: &str) -> bool {
    expression
        .clone()
        .omit_type_wrappers()
        .and_then(|expression| expression.as_static_value())
        .is_some_and(|value| value.as_string_constant() == Some(expected))
}

/// Returns whether `name` is a non-computed member name equal to `expected`.
fn is_literal_member_named(name: &AnyJsClassMemberName, expected: &str) -> bool {
    name.as_js_literal_member_name()
        .and_then(|name| name.name().ok())
        .is_some_and(|name| name == expected)
}

/// Returns whether `name` is the name of an error class: words that each start with an ASCII
/// uppercase letter followed by lowercase letters or digits, ending with `Error`, such as
/// `HttpError` or `HTTPError`.
fn is_error_class_name(name: &str) -> bool {
    let Some(prefix) = name.strip_suffix("Error") else {
        return false;
    };
    let bytes = prefix.as_bytes();
    bytes.first().is_none_or(u8::is_ascii_uppercase)
        && bytes.iter().all(|&byte| {
            byte != b'_' && matches!(lookup_byte(byte), Dispatch::IDT | Dispatch::DIG | Dispatch::ZER)
        })
}

/// Returns whether `name` starts with an uppercase character and ends with `Error`, in which case
/// it equals [`ExpectedClassName`].
fn is_valid_class_name(name: &str) -> bool {
    name.strip_suffix("Error").is_some_and(|stem| {
        stem.chars()
            .next()
            .is_none_or(|first| first.to_uppercase().eq([first]))
    })
}

/// Writes the expected name of an error class from its current name: the first character is
/// uppercased, and a trailing `error` in any case is replaced with `Error`, or `Error` is
/// appended. For example, `fooerror` becomes `FooError` and `foo` becomes `FooError`.
struct ExpectedClassName<'a>(&'a str);

impl biome_console::fmt::Display for ExpectedClassName<'_> {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        let name = self.0;
        let stem = name
            .len()
            .checked_sub("error".len())
            .and_then(|start| name.get(start..).map(|suffix| (start, suffix)))
            .filter(|(_, suffix)| suffix.eq_ignore_ascii_case("error"))
            .map_or(name, |(start, _)| &name[..start]);
        let mut chars = stem.chars();
        if let Some(first) = chars.next() {
            let mut buffer = [0; 4];
            for upper in first.to_uppercase() {
                fmt.write_str(upper.encode_utf8(&mut buffer))?;
            }
        }
        fmt.write_str(chars.as_str())?;
        fmt.write_str("Error")
    }
}

/// Global built-in errors whose constructor accepts the error options as its second argument,
/// sorted for binary search.
const BUILTIN_ERRORS_WITH_OPTIONS: &[&str] = &[
    "Error",
    "EvalError",
    "RangeError",
    "ReferenceError",
    "SyntaxError",
    "TypeError",
    "URIError",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_errors_with_options_are_sorted() {
        for pair in BUILTIN_ERRORS_WITH_OPTIONS.windows(2) {
            assert!(
                pair[0].cmp(pair[1]).is_lt(),
                "{} must come after {}",
                pair[0],
                pair[1]
            );
        }
    }
}
