use crate::{
    AnyJsCallArgument, AnyJsFunction, AnyJsFunctionBody, JsCallArguments, JsMethodClassMember,
    JsMethodObjectMember, JsStatementList, JsSyntaxToken,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult, TextRange};

declare_node_union! {
    pub AnyFunctionLike = AnyJsFunction | JsMethodObjectMember | JsMethodClassMember
}

impl AnyFunctionLike {
    pub fn body(&self) -> SyntaxResult<AnyJsFunctionBody> {
        match self {
            AnyFunctionLike::AnyJsFunction(js_function) => js_function.body(),
            AnyFunctionLike::JsMethodObjectMember(js_object_method) => js_object_method
                .body()
                .map(AnyJsFunctionBody::JsFunctionBody),
            AnyFunctionLike::JsMethodClassMember(js_class_method) => js_class_method
                .body()
                .map(AnyJsFunctionBody::JsFunctionBody),
        }
    }

    pub fn fat_arrow_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyFunctionLike::AnyJsFunction(any_js_function) => {
                if let Some(arrow_expression) = any_js_function.as_js_arrow_function_expression() {
                    arrow_expression.fat_arrow_token().ok()
                } else {
                    None
                }
            }
            AnyFunctionLike::JsMethodClassMember(_) | AnyFunctionLike::JsMethodObjectMember(_) => {
                None
            }
        }
    }

    pub fn function_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyFunctionLike::AnyJsFunction(any_js_function) => {
                any_js_function.function_token().ok().flatten()
            }
            AnyFunctionLike::JsMethodClassMember(_) | AnyFunctionLike::JsMethodObjectMember(_) => {
                None
            }
        }
    }

    pub fn is_generator(&self) -> bool {
        match self {
            AnyFunctionLike::AnyJsFunction(any_js_function) => any_js_function.is_generator(),
            AnyFunctionLike::JsMethodClassMember(method_class_member) => {
                method_class_member.star_token().is_some()
            }
            AnyFunctionLike::JsMethodObjectMember(method_obj_member) => {
                method_obj_member.star_token().is_some()
            }
        }
    }

    pub fn is_async(&self) -> bool {
        match self {
            AnyFunctionLike::AnyJsFunction(any_js_function) => any_js_function.is_async(),
            AnyFunctionLike::JsMethodClassMember(method_class_member) => {
                method_class_member.async_token().is_some()
            }
            AnyFunctionLike::JsMethodObjectMember(method_obj_member) => {
                method_obj_member.async_token().is_some()
            }
        }
    }

    pub fn name_range(&self) -> Option<TextRange> {
        match self {
            AnyFunctionLike::AnyJsFunction(js_function) => {
                js_function.id().ok().flatten().map(|id| id.range())
            }
            AnyFunctionLike::JsMethodObjectMember(js_object_method) => {
                js_object_method.name().ok().map(|name| name.range())
            }
            AnyFunctionLike::JsMethodClassMember(js_class_method) => {
                js_class_method.name().ok().map(|name| name.range())
            }
        }
    }

    pub fn statements(&self) -> Option<JsStatementList> {
        Some(match self {
            AnyFunctionLike::AnyJsFunction(any_js_function) => any_js_function
                .body()
                .ok()?
                .as_js_function_body()?
                .statements(),
            AnyFunctionLike::JsMethodClassMember(method_class_member) => {
                method_class_member.body().ok()?.statements()
            }
            AnyFunctionLike::JsMethodObjectMember(method_obj_member) => {
                method_obj_member.body().ok()?.statements()
            }
        })
    }
}

impl JsCallArguments {
    /// Get [AnyJsCallArgument] by its index inside the [crate::JsCallExpression] argument list.
    ///
    /// Each index inside `indices` should be unique qnd in-order.
    ///
    /// Supports a maximum of 16 indices to avoid stack overflow.
    pub fn get_arguments_by_index<const N: usize>(
        &self,
        indices: [usize; N],
    ) -> [Option<AnyJsCallArgument>; N] {
        debug_assert!(N <= 16);
        // assert there are no duplicates and they are in-order
        debug_assert!(indices.windows(2).all(|vs| vs[0] < vs[1]));

        const INIT: Option<AnyJsCallArgument> = None;
        let mut result = [INIT; N];
        let mut next = 0;
        for (i, arg) in self.args().into_iter().flatten().enumerate() {
            if i == indices[next] {
                result[next] = Some(arg);
                next += 1;
                if next == N {
                    break;
                }
            }
        }
        result
    }
}
