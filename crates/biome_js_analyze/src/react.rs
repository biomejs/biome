//! A series of AST utilities to work with the React library

pub mod hooks;

use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, AnyJsNamedImportSpecifier,
    AnyJsObjectMember, JsCallExpression, JsIdentifierBinding, JsImport, JsObjectExpression,
    JsPropertyObjectMember, JsxMemberName, JsxReferenceIdentifier,
};
use biome_rowan::{AstNode, AstSeparatedList};

/// A trait to share common logic among data structures that "mimic" react APIs
pub(crate) trait ReactApiCall {
    /// It scans the current props and returns the property that matches the passed name
    fn find_prop_by_name(&self, prop_name: &str) -> Option<JsPropertyObjectMember>;
}

/// A convenient data structure that returns the three arguments of the [React.createElement] call
///
///[React.createElement]: https://reactjs.org/docs/react-api.html#createelement
pub(crate) struct ReactCreateElementCall {
    /// The type of the react element
    pub(crate) element_type: AnyJsCallArgument,
    /// Optional props
    pub(crate) props: Option<JsObjectExpression>,
    /// Optional children
    pub(crate) children: Option<AnyJsExpression>,
}

impl ReactCreateElementCall {
    /// Checks if the current node is a possible `createElement` call.
    ///
    /// There are two cases:
    ///
    /// First case
    /// ```js
    /// React.createElement()
    /// ```
    /// We check if the node is a static member expression with the specific members. Also, if `React`
    /// has been imported in the current scope, we make sure that the binding `React` has been imported
    /// from the `"react"` module.
    ///
    /// Second case
    ///
    /// ```js
    /// createElement()
    /// ```
    ///
    /// The logic of this second case is very similar to the previous one, simply the node that we have
    /// to inspect is different.
    pub(crate) fn from_call_expression(
        call_expression: &JsCallExpression,
        model: &SemanticModel,
    ) -> Option<Self> {
        let callee = call_expression.callee().ok()?;
        let is_react_create_element =
            is_react_call_api(callee, model, ReactLibrary::React, "createElement");

        if is_react_create_element {
            let arguments = call_expression.arguments().ok()?.args();
            // React.createElement() should not be processed
            if !arguments.is_empty() {
                let mut iter = arguments.iter();
                let first_argument = if let Some(first_argument) = iter.next() {
                    first_argument.ok()?
                } else {
                    return None;
                };
                let second_argument =
                    iter.next()
                        .and_then(|argument| argument.ok())
                        .and_then(|argument| {
                            argument
                                .as_any_js_expression()?
                                .as_js_object_expression()
                                .cloned()
                        });
                let third_argument = iter
                    .next()
                    .and_then(|argument| argument.ok())
                    .and_then(|argument| argument.as_any_js_expression().cloned());

                Some(ReactCreateElementCall {
                    element_type: first_argument,
                    props: second_argument,
                    children: third_argument,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl ReactApiCall for ReactCreateElementCall {
    /// It scans the current props and returns the property that matches the passed name
    fn find_prop_by_name(&self, prop_name: &str) -> Option<JsPropertyObjectMember> {
        self.props.as_ref().and_then(|props| {
            let members = props.members();
            members.iter().find_map(|member| {
                let AnyJsObjectMember::JsPropertyObjectMember(property) = member.ok()? else {
                    return None;
                };
                let property_name = property.name().ok()?;

                let property_name = property_name.as_js_literal_member_name()?;
                if property_name.name().ok()? == prop_name {
                    Some(property)
                } else {
                    None
                }
            })
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ReactLibrary {
    React,
    ReactDOM,
}

impl ReactLibrary {
    pub const fn import_name(self) -> &'static str {
        match self {
            ReactLibrary::React => "react",
            ReactLibrary::ReactDOM => "react-dom",
        }
    }

    pub const fn global_name(self) -> &'static str {
        match self {
            ReactLibrary::React => "React",
            ReactLibrary::ReactDOM => "ReactDOM",
        }
    }
}

/// List of valid [`React` API]
///
/// [`React` API]: https://reactjs.org/docs/react-api.html
const VALID_REACT_API: [&str; 29] = [
    "Component",
    "PureComponent",
    "memo",
    "createElement",
    "cloneElement",
    "createFactory",
    "isValidElement",
    "Fragment",
    "createRef",
    "forwardRef",
    "lazy",
    "Suspense",
    "startTransition",
    "Children",
    "useEffect",
    "useLayoutEffect",
    "useInsertionEffect",
    "useCallback",
    "useMemo",
    "useImperativeHandle",
    "useState",
    "useContext",
    "useReducer",
    "useRef",
    "useDebugValue",
    "useDeferredValue",
    "useTransition",
    "useId",
    "useSyncExternalStore",
];

/// Checks if the current [JsCallExpression] is a potential [`React` API].
/// The function has accepts a `api_name` to check against
///
/// [`React` API]: https://reactjs.org/docs/react-api.html
pub(crate) fn is_react_call_api(
    expression: AnyJsExpression,
    model: &SemanticModel,
    lib: ReactLibrary,
    api_name: &str,
) -> bool {
    if matches!(lib, ReactLibrary::React) {
        // we bail straight away if the API doesn't exists in React
        debug_assert!(VALID_REACT_API.contains(&api_name));
    }

    let expr = expression.omit_parentheses();
    if let Some(callee) = AnyJsMemberExpression::cast_ref(expr.syntax()) {
        let Some(object) = callee.object().ok() else {
            return false;
        };
        let Some(reference) = object.omit_parentheses().as_js_reference_identifier() else {
            return false;
        };
        let Some(member_name) = callee.member_name() else {
            return false;
        };
        if member_name.text() != api_name {
            return false;
        }
        return match model.binding(&reference) {
            Some(decl) => is_react_export(&decl, lib),
            None => reference.has_name(lib.global_name()),
        };
    }

    if let Some(ident) = expr.as_js_reference_identifier() {
        return model
            .binding(&ident)
            .and_then(|it| is_named_react_export(&it, lib, api_name))
            .unwrap_or(false);
    }

    false
}

/// Checks if the node `JsxMemberName` is a react fragment.
///
/// e.g. `<React.Fragment>` is a fragment, but no `<React.StrictMode>`.
///
/// In case the `React` is a valid reference, the function checks if it is exported from the
/// `"react"` library
pub(crate) fn jsx_member_name_is_react_fragment(
    member_name: &JsxMemberName,
    model: &SemanticModel,
) -> Option<bool> {
    let object = member_name.object().ok()?;
    let member = member_name.member().ok()?;
    let object = object.as_jsx_reference_identifier()?;

    if member.value_token().ok()?.text_trimmed() != "Fragment" {
        return Some(false);
    }

    let lib = ReactLibrary::React;
    match model.binding(object) {
        Some(declaration) => Some(is_react_export(&declaration, lib)),
        None => Some(object.value_token().ok()?.text_trimmed() == lib.global_name()),
    }
}

/// Checks if the node `JsxReferenceIdentifier` is a react fragment.
///
/// e.g. `<Fragment>` is a fragment
///
/// In case the `Fragment` is a valid reference, the function checks if it is exported from the
/// `"react"` library
pub(crate) fn jsx_reference_identifier_is_fragment(
    name: &JsxReferenceIdentifier,
    model: &SemanticModel,
) -> Option<bool> {
    match model.binding(name) {
        Some(reference) => is_named_react_export(&reference, ReactLibrary::React, "Fragment"),
        None => {
            let value_token = name.value_token().ok()?;
            let is_fragment = value_token.text_trimmed() == "Fragment";
            Some(is_fragment)
        }
    }
}

fn is_react_export(binding: &Binding, lib: ReactLibrary) -> bool {
    binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| source.text() == lib.import_name())
}

fn is_named_react_export(binding: &Binding, lib: ReactLibrary, name: &str) -> Option<bool> {
    let ident = JsIdentifierBinding::cast_ref(binding.syntax())?;
    let import_specifier = ident.parent::<AnyJsNamedImportSpecifier>()?;
    let name_token = match &import_specifier {
        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(named_import) => {
            named_import.name().ok()?.value().ok()?
        }
        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(_) => ident.name_token().ok()?,
        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => {
            return Some(false);
        }
    };

    if name_token.text_trimmed() != name {
        return Some(false);
    }

    let import = import_specifier.import_clause()?.parent::<JsImport>()?;
    Some(import.source_text().ok()?.text() == lib.import_name())
}
