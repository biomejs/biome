use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding};
use biome_js_syntax::{
    AnyFunctionLike, AnyJsExpression, AnyJsMemberExpression, AnyJsNamedImportSpecifier, AnyTsName,
    AnyTsType, JsAwaitExpression, JsCallExpression, JsDoWhileStatement, JsForInStatement,
    JsForOfStatement, JsForStatement, JsGetterClassMember, JsGetterObjectMember, JsImport,
    JsLanguage, JsReferenceIdentifier, JsSetterClassMember, JsSetterObjectMember, JsSyntaxNode,
    JsSyntaxToken, JsWhileStatement, JsYieldExpression, T, TsReferenceType,
};
use biome_rowan::{
    AstNode, AstSeparatedList, SyntaxKindSet, TextRange, TextSize, WalkEvent, declare_node_union,
};
use biome_rule_options::no_late_event_control::NoLateEventControlOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

use crate::react::ReactLibrary;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow calling `preventDefault()`, `stopPropagation()`, or `stopImmediatePropagation()` after the browser has finished handling the event.
    ///
    /// When an event happens, the browser runs each event handler for it, one after another.
    /// `preventDefault()`, `stopPropagation()`, and `stopImmediatePropagation()` only work while
    /// the browser is still running these handlers. When a handler pauses, the browser moves on
    /// without waiting for it, so calling these methods afterwards does nothing: the browser still
    /// performs the default action, such as following a link, and other handlers still receive
    /// the event.
    ///
    /// Code in a handler runs too late in these cases:
    ///
    /// - The code comes after an `await` or a `for await` loop in an `async` function.
    /// - The code is in a function nested inside the handler, such as a `setTimeout()` or `.then()` callback. The nested function can run after the handler has finished.
    /// - The handler is a generator function (`function*`). Calling a generator function doesn't run its body.
    ///
    /// Call these methods directly in the handler, before the first `await`.
    ///
    /// The rule checks calls on a function parameter whose name looks like an event, such as `e`,
    /// `event`, `evt`, or a camelCase name ending with `Event` like `clickEvent`. In TypeScript,
    /// it also checks parameters with an event type, such as `MouseEvent` or React's
    /// `SyntheticEvent`, and ignores parameters with a type that isn't an event, such as `string`.
    ///
    /// The rule has these limitations:
    ///
    /// - Every call inside a nested function is reported, even when the nested function runs right away, like the callback of `array.map()`. Move the call to the handler itself to make it clear that it runs in time.
    /// - The rule doesn't follow the order in which code runs. A call is reported when an `await` appears earlier in the handler, even if the `await` is in a branch that never runs together with the call.
    /// - When the event is passed to another function, calls inside that function are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// element.addEventListener("click", async (event) => {
    ///     await save();
    ///     event.preventDefault();
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// element.addEventListener("click", (event) => {
    ///     setTimeout(() => {
    ///         event.stopPropagation();
    ///     });
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// element.onclick = function* (event) {
    ///     event.preventDefault();
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// element.addEventListener("click", async (event) => {
    ///     event.preventDefault();
    ///     await save();
    /// });
    ///
    /// element.addEventListener("click", (event) => {
    ///     event.stopPropagation();
    ///     setTimeout(() => {
    ///         update();
    ///     });
    /// });
    /// ```
    ///
    pub NoLateEventControl {
        version: "next",
        name: "noLateEventControl",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-late-event-control").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoLateEventControl {
    type Query = Semantic<JsCallExpression>;
    type State = LateEventControl;
    type Signals = Option<Self::State>;
    type Options = NoLateEventControlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        let callee =
            AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
        let method = EventControlMethod::from_name(callee.member_name()?.text())?;
        let event = callee
            .object()
            .ok()?
            .omit_type_wrappers()?
            .as_js_reference_identifier()?;

        let binding = model.binding(&event)?.tree();
        let declaration = binding.declaration()?;
        let parameter = declaration
            .parent_binding_pattern_declaration()
            .unwrap_or_else(|| declaration.clone());
        if !parameter.is_parameter_like() {
            return None;
        }
        let handler = parameter
            .syntax()
            .ancestors()
            .find_map(AnyFunctionBoundary::cast)?;
        if !is_event_parameter(model, &binding, &declaration) {
            return None;
        }

        let enclosing_function = call
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(AnyFunctionBoundary::cast)?;
        let kind = if enclosing_function != handler {
            LateEventControlKind::NestedFunction
        } else if handler.is_generator() {
            LateEventControlKind::Generator
        } else if handler.is_async() {
            LateEventControlKind::AfterPause(find_earlier_pause(call, &handler)?)
        } else {
            return None;
        };

        Some(LateEventControl {
            kind,
            method,
            event: event.value_token().ok()?,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let LateEventControl {
            kind,
            method,
            event,
        } = state;
        let event = event.text_trimmed();
        let call = markup! { <Emphasis>{event}"."{method}"()"</Emphasis> };
        let diagnostic = match kind {
            LateEventControlKind::AfterPause(pause) => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { {call}" is called after the event handler pauses." },
            )
            .detail(pause, markup! { "The event handler pauses here." })
            .note(markup! {
                "When the handler pauses, the browser finishes handling the event without waiting for it, so this call has no effect."
            })
            .note(markup! {
                "Call "{call}" before the event handler pauses."
            }),
            LateEventControlKind::NestedFunction => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { {call}" is called inside a function nested in the event handler." },
            )
            .note(markup! {
                "The nested function may run after the browser finishes handling the event, when this call has no effect."
            })
            .note(markup! {
                "Call "{call}" directly in the event handler, before any delayed work."
            }),
            LateEventControlKind::Generator => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { {call}" is called inside a generator function." },
            )
            .note(markup! {
                "Calling a generator function doesn't run its body, so this call doesn't happen while the browser handles the event."
            })
            .note(markup! {
                "Use a regular function as the event handler."
            }),
        };
        Some(diagnostic)
    }
}

pub struct LateEventControl {
    kind: LateEventControlKind,
    method: EventControlMethod,
    /// The name token of the event parameter, as written at the call.
    event: JsSyntaxToken,
}

pub enum LateEventControlKind {
    /// The call runs after the handler pauses at the `await`, `yield`, or `for await` loop at
    /// this range.
    AfterPause(TextRange),
    /// The call is inside a function nested in the handler.
    NestedFunction,
    /// The handler is a generator function.
    Generator,
}

#[derive(Clone, Copy)]
pub enum EventControlMethod {
    PreventDefault,
    StopImmediatePropagation,
    StopPropagation,
}

declare_node_union! {
    /// A function that has its own body. An `await` or `yield` only pauses the function that
    /// directly contains it.
    AnyFunctionBoundary = AnyFunctionLike
        | JsGetterClassMember
        | JsGetterObjectMember
        | JsSetterClassMember
        | JsSetterObjectMember
}

impl AnyFunctionBoundary {
    fn is_generator(&self) -> bool {
        match self {
            Self::AnyFunctionLike(function) => function.is_generator(),
            Self::JsGetterClassMember(_)
            | Self::JsGetterObjectMember(_)
            | Self::JsSetterClassMember(_)
            | Self::JsSetterObjectMember(_) => false,
        }
    }

    fn is_async(&self) -> bool {
        match self {
            Self::AnyFunctionLike(function) => function.is_async(),
            Self::JsGetterClassMember(_)
            | Self::JsGetterObjectMember(_)
            | Self::JsSetterClassMember(_)
            | Self::JsSetterObjectMember(_) => false,
        }
    }
}

declare_node_union! {
    AnyLoopStatement = JsDoWhileStatement
        | JsForInStatement
        | JsForOfStatement
        | JsForStatement
        | JsWhileStatement
}

impl EventControlMethod {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "preventDefault" => Some(Self::PreventDefault),
            "stopImmediatePropagation" => Some(Self::StopImmediatePropagation),
            "stopPropagation" => Some(Self::StopPropagation),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for EventControlMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::PreventDefault => "preventDefault",
            Self::StopImmediatePropagation => "stopImmediatePropagation",
            Self::StopPropagation => "stopPropagation",
        })
    }
}

/// Whether a type is known to be an event type.
#[derive(Clone, Copy, Eq, PartialEq)]
enum EventType {
    Event,
    NotEvent,
    Unknown,
}

/// The number of nested types and named type declarations followed when deciding whether a
/// parameter type is an event type. It also stops the search when type aliases, interfaces, or
/// classes refer to each other in a cycle.
const MAX_TYPE_DEPTH: u8 = 8;

const PAUSE_KINDS: SyntaxKindSet<JsLanguage> =
    JsAwaitExpression::KIND_SET.union(JsYieldExpression::KIND_SET);

/// The nodes [pause_points] needs to see: the pauses, the loops that can be `for await` loops,
/// and the nested functions it skips.
const PAUSE_POINT_WALK_KINDS: SyntaxKindSet<JsLanguage> = PAUSE_KINDS
    .union(JsForOfStatement::KIND_SET)
    .union(AnyFunctionBoundary::KIND_SET);

/// Returns whether the parameter that declares `binding` holds an event.
///
/// A type annotation that is known to be an event type, or known not to be one, decides the
/// result. Otherwise, the parameter name decides.
fn is_event_parameter(
    model: &SemanticModel,
    binding: &AnyJsIdentifierBinding,
    declaration: &AnyJsBindingDeclaration,
) -> bool {
    let parameter = match declaration {
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => Some(parameter.clone()),
        AnyJsBindingDeclaration::TsPropertyParameter(parameter) => parameter
            .formal_parameter()
            .ok()
            .and_then(|parameter| parameter.as_js_formal_parameter().cloned()),
        _ => None,
    };
    let event_type = parameter
        .and_then(|parameter| {
            let ty = parameter.type_annotation()?.ty().ok()?;
            let event_type = classify_type(model, &ty, MAX_TYPE_DEPTH);
            // An optional parameter can also be `undefined`, so it's only known to be an event
            // when it's present.
            Some(
                if parameter.question_mark_token().is_some() && event_type == EventType::Event {
                    EventType::Unknown
                } else {
                    event_type
                },
            )
        })
        .unwrap_or(EventType::Unknown);

    match event_type {
        EventType::Event => true,
        EventType::NotEvent => false,
        EventType::Unknown => binding
            .name_token()
            .is_ok_and(|name| is_event_name(name.text_trimmed())),
    }
}

/// Returns whether `name` is `e`, `event`, `evt`, or a camelCase name that ends with `Event`,
/// such as `clickEvent`.
fn is_event_name(name: &str) -> bool {
    if matches!(name, "e" | "event" | "evt") {
        return true;
    }
    let Some(prefix) = name.strip_suffix("Event") else {
        return false;
    };
    let mut bytes = prefix.bytes();
    bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        && bytes.all(|byte| {
            byte != b'_'
                && matches!(
                    lookup_byte(byte),
                    Dispatch::IDT | Dispatch::ZER | Dispatch::DIG
                )
        })
}

fn classify_type(model: &SemanticModel, ty: &AnyTsType, depth: u8) -> EventType {
    let Some(depth) = depth.checked_sub(1) else {
        return EventType::Unknown;
    };
    match ty {
        AnyTsType::TsParenthesizedType(ty) => ty
            .ty()
            .map_or(EventType::Unknown, |ty| classify_type(model, &ty, depth)),
        AnyTsType::TsTypeOperatorType(ty) => {
            if ty
                .operator_token()
                .is_ok_and(|operator| operator.kind() == T![readonly])
            {
                ty.ty()
                    .map_or(EventType::Unknown, |ty| classify_type(model, &ty, depth))
            } else {
                EventType::Unknown
            }
        }
        AnyTsType::TsReferenceType(ty) => classify_type_reference(model, ty, depth),
        AnyTsType::TsUnionType(ty) => {
            let mut has_event = false;
            let mut has_not_event = false;
            for ty in ty.types().iter() {
                match ty.map_or(EventType::Unknown, |ty| classify_type(model, &ty, depth)) {
                    EventType::Event => has_event = true,
                    EventType::NotEvent => has_not_event = true,
                    EventType::Unknown => return EventType::Unknown,
                }
            }
            match (has_event, has_not_event) {
                (true, false) => EventType::Event,
                (false, _) => EventType::NotEvent,
                (true, true) => EventType::Unknown,
            }
        }
        AnyTsType::TsIntersectionType(ty) => combine_intersection(
            ty.types()
                .iter()
                .map(|ty| ty.map_or(EventType::Unknown, |ty| classify_type(model, &ty, depth))),
        ),
        AnyTsType::TsArrayType(_)
        | AnyTsType::TsBigintLiteralType(_)
        | AnyTsType::TsBigintType(_)
        | AnyTsType::TsBooleanLiteralType(_)
        | AnyTsType::TsBooleanType(_)
        | AnyTsType::TsConstructorType(_)
        | AnyTsType::TsFunctionType(_)
        | AnyTsType::TsNeverType(_)
        | AnyTsType::TsNullLiteralType(_)
        | AnyTsType::TsNumberLiteralType(_)
        | AnyTsType::TsNumberType(_)
        | AnyTsType::TsObjectType(_)
        | AnyTsType::TsStringLiteralType(_)
        | AnyTsType::TsStringType(_)
        | AnyTsType::TsSymbolType(_)
        | AnyTsType::TsTemplateLiteralType(_)
        | AnyTsType::TsTupleType(_)
        | AnyTsType::TsUndefinedType(_)
        | AnyTsType::TsVoidType(_) => EventType::NotEvent,
        _ => EventType::Unknown,
    }
}

fn classify_type_reference(
    model: &SemanticModel,
    reference: &TsReferenceType,
    depth: u8,
) -> EventType {
    match reference.name() {
        Ok(AnyTsName::JsReferenceIdentifier(name)) => classify_type_name(model, &name, depth),
        Ok(AnyTsName::TsQualifiedName(name)) => {
            // Only `Namespace.Name`, where `Namespace` is an import, is recognized.
            let (Ok(AnyTsName::JsReferenceIdentifier(namespace)), Ok(member)) =
                (name.left(), name.right())
            else {
                return EventType::Unknown;
            };
            let Ok(member) = member.value_token() else {
                return EventType::Unknown;
            };
            let Some(binding) = model.binding(&namespace) else {
                return EventType::Unknown;
            };
            match binding.tree().declaration() {
                Some(
                    AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_)
                    | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_),
                ) => classify_imported_type(&binding, member.text_trimmed(), None),
                _ => EventType::Unknown,
            }
        }
        Err(_) => EventType::Unknown,
    }
}

fn classify_type_name(model: &SemanticModel, name: &JsReferenceIdentifier, depth: u8) -> EventType {
    let Some(depth) = depth.checked_sub(1) else {
        return EventType::Unknown;
    };
    let Some(binding) = model.binding(name) else {
        return name.value_token().map_or(EventType::Unknown, |name| {
            if is_global_event_type(name.text_trimmed()) {
                EventType::Event
            } else {
                EventType::Unknown
            }
        });
    };
    let Some(declaration) = binding.tree().declaration() else {
        return EventType::Unknown;
    };
    match declaration {
        AnyJsBindingDeclaration::JsNamedImportSpecifier(specifier) => {
            classify_named_import(&binding, &AnyJsNamedImportSpecifier::from(specifier))
        }
        AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(specifier) => {
            classify_named_import(&binding, &AnyJsNamedImportSpecifier::from(specifier))
        }
        AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => {
            // The default export of a module can't be one of the named event types.
            if binding
                .tree()
                .name_token()
                .is_ok_and(|name| is_global_event_type(name.text_trimmed()))
            {
                EventType::NotEvent
            } else {
                EventType::Unknown
            }
        }
        AnyJsBindingDeclaration::TsTypeAliasDeclaration(alias) => alias
            .ty()
            .map_or(EventType::Unknown, |ty| classify_type(model, &ty, depth)),
        AnyJsBindingDeclaration::TsTypeParameter(parameter) => parameter
            .constraint()
            .and_then(|constraint| constraint.ty().ok())
            .map_or(EventType::Unknown, |ty| classify_type(model, &ty, depth)),
        AnyJsBindingDeclaration::TsInterfaceDeclaration(interface) => {
            let Some(extends) = interface.extends_clause() else {
                return EventType::NotEvent;
            };
            combine_intersection(extends.types().iter().map(|ty| {
                ty.map_or(EventType::Unknown, |ty| {
                    classify_type_reference(model, &ty, depth)
                })
            }))
        }
        AnyJsBindingDeclaration::JsClassDeclaration(class) => {
            let Some(extends) = class.extends_clause() else {
                return EventType::NotEvent;
            };
            match extends.super_class() {
                Ok(AnyJsExpression::JsIdentifierExpression(super_class)) => {
                    super_class.name().map_or(EventType::Unknown, |name| {
                        classify_type_name(model, &name, depth)
                    })
                }
                _ => EventType::Unknown,
            }
        }
        _ => EventType::Unknown,
    }
}

fn classify_named_import(binding: &Binding, specifier: &AnyJsNamedImportSpecifier) -> EventType {
    let Some(imported_name) = specifier.imported_name() else {
        return EventType::Unknown;
    };
    let local_name = binding.tree().name_token().ok();
    classify_imported_type(
        binding,
        imported_name.text_trimmed(),
        local_name.as_ref().map(JsSyntaxToken::text_trimmed),
    )
}

/// Classifies the type `imported_name` imported from the module that declares `binding`.
///
/// Event types imported from React are events. Other imports that reuse the name of an event
/// type, either as `imported_name` or as the `local_name` they're renamed to, refer to some
/// other type.
fn classify_imported_type(
    binding: &Binding,
    imported_name: &str,
    local_name: Option<&str>,
) -> EventType {
    let Some(source) = binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
    else {
        return EventType::Unknown;
    };
    if ReactLibrary::React.import_names().contains(&source.text()) {
        return if is_react_event_type(imported_name) {
            EventType::Event
        } else {
            EventType::NotEvent
        };
    }
    if is_global_event_type(imported_name) || local_name.is_some_and(is_global_event_type) {
        EventType::NotEvent
    } else {
        EventType::Unknown
    }
}

/// Combines the members of an intersection type, or the types an interface extends. The result
/// is an event type when one of the members is.
fn combine_intersection(types: impl Iterator<Item = EventType>) -> EventType {
    let mut has_event = false;
    for ty in types {
        match ty {
            EventType::Event => has_event = true,
            EventType::NotEvent => {}
            // An unknown member can be `any`, which makes the whole intersection `any`.
            EventType::Unknown => return EventType::Unknown,
        }
    }
    if has_event {
        EventType::Event
    } else {
        EventType::NotEvent
    }
}

/// Returns the range of an `await`, `yield`, or `for await` loop in `handler` that pauses it
/// before `call` runs.
///
/// A pause counts when it completes before `call` does, or when `call` is in the condition or
/// body of a loop that pauses, because the next iteration then runs after the pause.
fn find_earlier_pause(call: &JsCallExpression, handler: &AnyFunctionBoundary) -> Option<TextRange> {
    let call_range = call.syntax().text_trimmed_range();
    let earlier_pause = pause_points(handler, call_range.end()).find(|pause| {
        let pause_range = pause.text_trimmed_range();
        pause_range.end() <= call_range.end() && !pause_range.contains_range(call_range)
    });
    if let Some(pause) = earlier_pause {
        return Some(pause.text_trimmed_range());
    }

    let mut child = call.syntax().clone();
    for ancestor in call.syntax().ancestors().skip(1) {
        if ancestor == *handler.syntax() {
            break;
        }
        if let Some(loop_statement) = AnyLoopStatement::cast_ref(&ancestor)
            && is_repeated_loop_part(&loop_statement, &child)
            && let Some(pause) =
                pause_points(&loop_statement, ancestor.text_trimmed_range().end()).next()
        {
            return Some(pause.text_trimmed_range());
        }
        child = ancestor;
    }
    None
}

/// Returns whether `child`, a direct child of `loop_statement`, runs again on every iteration.
fn is_repeated_loop_part(loop_statement: &AnyLoopStatement, child: &JsSyntaxNode) -> bool {
    match loop_statement {
        AnyLoopStatement::JsDoWhileStatement(_) | AnyLoopStatement::JsWhileStatement(_) => true,
        AnyLoopStatement::JsForStatement(loop_statement) => loop_statement
            .initializer()
            .is_none_or(|initializer| initializer.syntax() != child),
        // The iterated expression after `in` or `of` runs once, before the loop starts.
        AnyLoopStatement::JsForInStatement(loop_statement) => loop_statement
            .expression()
            .is_ok_and(|expression| expression.syntax() != child),
        AnyLoopStatement::JsForOfStatement(loop_statement) => loop_statement
            .expression()
            .is_ok_and(|expression| expression.syntax() != child),
    }
}

/// Returns the `await` expressions, `yield` expressions, and `for await` loops in `root` that
/// start before `end`, in source order.
///
/// Functions nested in `root` are skipped, because pausing them doesn't pause `root`.
fn pause_points(
    root: &impl AstNode<Language = JsLanguage>,
    end: TextSize,
) -> impl Iterator<Item = JsSyntaxNode> + '_ {
    let root = root.syntax();
    let mut preorder = root.preorder_of_kinds(PAUSE_POINT_WALK_KINDS);
    std::iter::from_fn(move || {
        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if node.text_trimmed_range().start() >= end {
                return None;
            }
            if node != *root && AnyFunctionBoundary::can_cast(node.kind()) {
                preorder.skip_subtree();
                continue;
            }
            if PAUSE_KINDS.matches(node.kind())
                || JsForOfStatement::cast_ref(&node)
                    .is_some_and(|loop_statement| loop_statement.await_token().is_some())
            {
                return Some(node);
            }
        }
        None
    })
}

fn is_global_event_type(name: &str) -> bool {
    GLOBAL_EVENT_TYPES.binary_search(&name).is_ok()
}

fn is_react_event_type(name: &str) -> bool {
    REACT_EVENT_TYPES.binary_search(&name).is_ok()
}

/// Event types that are available without an import, from the DOM and from React's global
/// type declarations. Sorted for binary search.
const GLOBAL_EVENT_TYPES: &[&str] = &[
    "AnimationEvent",
    "AnimationPlaybackEvent",
    "AudioProcessingEvent",
    "BaseSyntheticEvent",
    "BeforeUnloadEvent",
    "BlobEvent",
    "ChangeEvent",
    "ClipboardEvent",
    "CloseEvent",
    "CommandEvent",
    "CompositionEvent",
    "ContentVisibilityAutoStateChangeEvent",
    "CookieChangeEvent",
    "CustomEvent",
    "DeviceMotionEvent",
    "DeviceOrientationEvent",
    "DragEvent",
    "ErrorEvent",
    "Event",
    "ExtendableCookieChangeEvent",
    "ExtendableEvent",
    "ExtendableMessageEvent",
    "FetchEvent",
    "FocusEvent",
    "FontFaceSetLoadEvent",
    "FormDataEvent",
    "FormEvent",
    "GPUUncapturedErrorEvent",
    "GamepadEvent",
    "HashChangeEvent",
    "IDBVersionChangeEvent",
    "InputEvent",
    "InvalidEvent",
    "KeyboardEvent",
    "MIDIConnectionEvent",
    "MIDIMessageEvent",
    "MediaEncryptedEvent",
    "MediaKeyMessageEvent",
    "MediaQueryListEvent",
    "MediaStreamTrackEvent",
    "MessageEvent",
    "MouseEvent",
    "NavigateEvent",
    "NavigationCurrentEntryChangeEvent",
    "NotificationEvent",
    "OfflineAudioCompletionEvent",
    "PageRevealEvent",
    "PageSwapEvent",
    "PageTransitionEvent",
    "PaymentMethodChangeEvent",
    "PaymentRequestUpdateEvent",
    "PictureInPictureEvent",
    "PointerEvent",
    "PopStateEvent",
    "ProgressEvent",
    "PromiseRejectionEvent",
    "PushEvent",
    "PushSubscriptionChangeEvent",
    "RTCDTMFToneChangeEvent",
    "RTCDataChannelEvent",
    "RTCErrorEvent",
    "RTCPeerConnectionIceErrorEvent",
    "RTCPeerConnectionIceEvent",
    "RTCTrackEvent",
    "RTCTransformEvent",
    "SecurityPolicyViolationEvent",
    "SpeechRecognitionErrorEvent",
    "SpeechRecognitionEvent",
    "SpeechSynthesisErrorEvent",
    "SpeechSynthesisEvent",
    "StorageEvent",
    "SubmitEvent",
    "SyntheticEvent",
    "TaskPriorityChangeEvent",
    "TextEvent",
    "ToggleEvent",
    "TouchEvent",
    "TrackEvent",
    "TransitionEvent",
    "UIEvent",
    "WebGLContextEvent",
    "WheelEvent",
];

/// Event types exported by React. Sorted for binary search.
const REACT_EVENT_TYPES: &[&str] = &[
    "AnimationEvent",
    "BaseSyntheticEvent",
    "ChangeEvent",
    "ClipboardEvent",
    "CompositionEvent",
    "DragEvent",
    "FocusEvent",
    "FormEvent",
    "InputEvent",
    "InvalidEvent",
    "KeyboardEvent",
    "MouseEvent",
    "PointerEvent",
    "SubmitEvent",
    "SyntheticEvent",
    "ToggleEvent",
    "TouchEvent",
    "TransitionEvent",
    "UIEvent",
    "WheelEvent",
];

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_sorted(names: &[&str]) {
        for pair in names.windows(2) {
            assert!(
                pair[0] < pair[1],
                "`{}` must come before `{}`",
                pair[1],
                pair[0]
            );
        }
    }

    #[test]
    fn global_event_types_are_sorted() {
        assert_sorted(GLOBAL_EVENT_TYPES);
    }

    #[test]
    fn react_event_types_are_sorted() {
        assert_sorted(REACT_EVENT_TYPES);
    }
}
