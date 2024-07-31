use crate::ts::bogus::bogus_type::FormatTsBogusType;
use crate::ts::module::import_type::FormatTsImportType;
use crate::ts::types::any_type::FormatTsAnyType;
use crate::ts::types::array_type::FormatTsArrayType;
use crate::ts::types::bigint_literal_type::FormatTsBigintLiteralType;
use crate::ts::types::bigint_type::FormatTsBigintType;
use crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType;
use crate::ts::types::boolean_type::FormatTsBooleanType;
use crate::ts::types::conditional_type::FormatTsConditionalType;
use crate::ts::types::constructor_type::FormatTsConstructorType;
use crate::ts::types::function_type::FormatTsFunctionType;
use crate::ts::types::indexed_access_type::FormatTsIndexedAccessType;
use crate::ts::types::infer_type::FormatTsInferType;
use crate::ts::types::intersection_type::FormatTsIntersectionType;
use crate::ts::types::mapped_type::FormatTsMappedType;
use crate::ts::types::never_type::FormatTsNeverType;
use crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType;
use crate::ts::types::null_literal_type::FormatTsNullLiteralType;
use crate::ts::types::number_literal_type::FormatTsNumberLiteralType;
use crate::ts::types::number_type::FormatTsNumberType;
use crate::ts::types::object_type::FormatTsObjectType;
use crate::ts::types::parenthesized_type::FormatTsParenthesizedType;
use crate::ts::types::reference_type::FormatTsReferenceType;
use crate::ts::types::string_literal_type::FormatTsStringLiteralType;
use crate::ts::types::string_type::FormatTsStringType;
use crate::ts::types::symbol_type::FormatTsSymbolType;
use crate::ts::types::template_literal_type::FormatTsTemplateLiteralType;
use crate::ts::types::this_type::FormatTsThisType;
use crate::ts::types::tuple_type::FormatTsTupleType;
use crate::ts::types::type_operator_type::FormatTsTypeOperatorType;
use crate::ts::types::typeof_type::FormatTsTypeofType;
use crate::ts::types::undefined_type::FormatTsUndefinedType;
use crate::ts::types::union_type::FormatTsUnionType;
use crate::ts::types::unknown_type::FormatTsUnknownType;
use crate::ts::types::void_type::FormatTsVoidType;
use crate::JsCommentStyle;
use crate::{js::auxiliary::metavariable::FormatJsMetavariable, prelude::*};
use biome_formatter::{comments::CommentStyle, write, FormatRuleWithOptions};
use biome_js_syntax::{AnyTsType, JsLanguage, TsUnionType, TsUnionTypeVariantList};
use biome_rowan::{AstSeparatedElement, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub struct FormatTsUnionTypeVariantList {
    should_hug: bool,
}

impl FormatRuleWithOptions<TsUnionTypeVariantList> for FormatTsUnionTypeVariantList {
    type Options = bool;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.should_hug = options;
        self
    }
}

impl FormatRule<TsUnionTypeVariantList> for FormatTsUnionTypeVariantList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsUnionTypeVariantList, f: &mut JsFormatter) -> FormatResult<()> {
        let last_index = node.len().saturating_sub(1);

        f.join_with(space())
            .entries(
                node.elements()
                    .enumerate()
                    .map(|(index, item)| FormatTypeVariant {
                        last: index == last_index,
                        list: node,
                        element: item,
                        should_hug: self.should_hug,
                    }),
            )
            .finish()
    }
}

pub struct FormatTypeVariant<'a> {
    last: bool,
    should_hug: bool,
    element: AstSeparatedElement<JsLanguage, AnyTsType>,
    list: &'a TsUnionTypeVariantList,
}

impl Format<JsFormatContext> for FormatTypeVariant<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let separator = self.element.trailing_separator()?;

        let node = self.element.node()?;

        let is_suppressed = is_type_suppressed(node, self.list, f.comments());

        // This is a hack: It by passes the regular format node to only format the node without its comments.
        let format_node = format_with(|f: &mut JsFormatter| {
            if is_suppressed {
                write!(f, [format_suppressed_node(node.syntax()).skip_comments()])
            } else {
                match node {
                    AnyTsType::TsAnyType(ty) => FormatTsAnyType.fmt_node(ty, f),
                    AnyTsType::TsArrayType(ty) => FormatTsArrayType.fmt_node(ty, f),
                    AnyTsType::TsBigintLiteralType(ty) => FormatTsBigintLiteralType.fmt_node(ty, f),
                    AnyTsType::TsBigintType(ty) => FormatTsBigintType.fmt_node(ty, f),
                    AnyTsType::TsBooleanLiteralType(ty) => {
                        FormatTsBooleanLiteralType.fmt_node(ty, f)
                    }
                    AnyTsType::TsBooleanType(ty) => FormatTsBooleanType.fmt_node(ty, f),
                    AnyTsType::TsConditionalType(ty) => FormatTsConditionalType.fmt_node(ty, f),
                    AnyTsType::TsConstructorType(ty) => FormatTsConstructorType.fmt_node(ty, f),
                    AnyTsType::TsFunctionType(ty) => FormatTsFunctionType.fmt_node(ty, f),
                    AnyTsType::TsImportType(ty) => FormatTsImportType.fmt_node(ty, f),
                    AnyTsType::TsIndexedAccessType(ty) => FormatTsIndexedAccessType.fmt_node(ty, f),
                    AnyTsType::TsInferType(ty) => FormatTsInferType.fmt_node(ty, f),
                    AnyTsType::TsIntersectionType(ty) => FormatTsIntersectionType.fmt_node(ty, f),
                    AnyTsType::TsMappedType(ty) => FormatTsMappedType.fmt_node(ty, f),
                    AnyTsType::TsNeverType(ty) => FormatTsNeverType.fmt_node(ty, f),
                    AnyTsType::TsNonPrimitiveType(ty) => FormatTsNonPrimitiveType.fmt_node(ty, f),
                    AnyTsType::TsNullLiteralType(ty) => FormatTsNullLiteralType.fmt_node(ty, f),
                    AnyTsType::TsNumberLiteralType(ty) => FormatTsNumberLiteralType.fmt_node(ty, f),
                    AnyTsType::TsNumberType(ty) => FormatTsNumberType.fmt_node(ty, f),
                    AnyTsType::TsObjectType(ty) => FormatTsObjectType.fmt_node(ty, f),
                    AnyTsType::TsParenthesizedType(ty) => FormatTsParenthesizedType.fmt_node(ty, f),
                    AnyTsType::TsReferenceType(ty) => FormatTsReferenceType.fmt_node(ty, f),
                    AnyTsType::TsStringLiteralType(ty) => FormatTsStringLiteralType.fmt_node(ty, f),
                    AnyTsType::TsStringType(ty) => FormatTsStringType.fmt_node(ty, f),
                    AnyTsType::TsSymbolType(ty) => FormatTsSymbolType.fmt_node(ty, f),
                    AnyTsType::TsTemplateLiteralType(ty) => {
                        FormatTsTemplateLiteralType.fmt_node(ty, f)
                    }
                    AnyTsType::TsThisType(ty) => FormatTsThisType.fmt_node(ty, f),
                    AnyTsType::TsTupleType(ty) => FormatTsTupleType.fmt_node(ty, f),
                    AnyTsType::TsTypeOperatorType(ty) => FormatTsTypeOperatorType.fmt_node(ty, f),
                    AnyTsType::TsTypeofType(ty) => FormatTsTypeofType.fmt_node(ty, f),
                    AnyTsType::TsUndefinedType(ty) => FormatTsUndefinedType.fmt_node(ty, f),
                    AnyTsType::TsUnionType(ty) => FormatTsUnionType.fmt_node(ty, f),
                    AnyTsType::TsUnknownType(ty) => FormatTsUnknownType.fmt_node(ty, f),
                    AnyTsType::TsVoidType(ty) => FormatTsVoidType.fmt_node(ty, f),
                    AnyTsType::TsBogusType(ty) => FormatTsBogusType.fmt(ty, f),
                    AnyTsType::JsMetavariable(ty) => FormatJsMetavariable.fmt_node(ty, f),
                }
            }
        });

        write!(f, [format_leading_comments(node.syntax())])?;

        if self.should_hug {
            write!(f, [format_node])?;
        } else {
            write!(f, [align(2, &format_node)])?;
        }

        // There are a few special cases for nodes which handle their own
        // dangling comments:
        // - Mapped types place the dangling comments as _leading_ comments for
        // the type:
        //     {
        //       // This is a dangling comment, formatted as a leading comment
        //       [foo in keyof Foo]: T
        //     }
        // - Other object like types format their own comments _only when there
        // are no members in the type_:
        //     {
        //       // This is a dangling comment, formatted inside JsObjectLike
        //     }
        // - Empty tuple types also format their own dangling comments
        //     [
        //       // This is a dangling comment, formatted inside TsTupleType
        //     ]
        // Attempting to format any of these dangling comments again results
        // in double printing, and often invalid syntax, such as:
        //     const t: {
        //       // Hello
        //       [foo in keyof Foo]: T
        //     } = 1;
        // Would get formatted to:
        //     const t: {
        //       // Hello
        //       [foo in keyof Foo]: T
        //     }// Hello = 1;
        // This check prevents that double printing from happening. There may
        // be a better place for it in the long term (maybe just always ensure
        // that a comment hasn't already been formatted before writing it?),
        // but this covers the majority of these cases already.
        let has_already_formatted_dangling_comments = match node {
            AnyTsType::TsMappedType(_) => true,
            AnyTsType::TsObjectType(object) => object.members().is_empty(),
            AnyTsType::TsTupleType(tuple) => tuple.elements().is_empty(),
            _ => false,
        };

        if !is_suppressed && has_already_formatted_dangling_comments {
            write!(f, [format_dangling_comments(node.syntax())])?;
        }

        write!(f, [format_trailing_comments(node.syntax())])?;

        if let Some(token) = separator {
            if self.last {
                write!(f, [format_removed(token)])?;
            } else {
                if self.should_hug {
                    write!(f, [space()])?;
                } else {
                    write!(f, [soft_line_break_or_space()])?;
                }
                write![f, [token.format()]]?;
            }
        }

        Ok(())
    }
}

fn is_type_suppressed(
    ty: &AnyTsType,
    list: &TsUnionTypeVariantList,
    comments: &JsComments,
) -> bool {
    comments.mark_suppression_checked(ty.syntax());

    if let AnyTsType::TsUnionType(union) = ty {
        // If the union isn't empty, than the suppression applies to the first variant
        if !union.types().is_empty() {
            return false;
        }
    }

    // Otherwise check if the node has a suppression in its leading or dangling comments
    // before then checking the previous variants trailing "OwnLine" comments (comments that are on their own line)
    let leading_dangling = comments
        .leading_comments(ty.syntax())
        .iter()
        .chain(comments.dangling_comments(ty.syntax()));

    for comment in leading_dangling {
        if JsCommentStyle::is_suppression(comment.piece().text()) {
            return true;
        }
    }

    for comment in comments
        .trailing_comments(ty.syntax())
        .iter()
        .take_while(|comment| comment.lines_before() == 0)
    {
        if JsCommentStyle::is_suppression(comment.piece().text()) {
            return true;
        }
    }

    // Test if the preceding node as a trailing own line comment that is a suppression
    if let Some(preceding_variant) = ty.syntax().prev_sibling() {
        comments
            .trailing_comments(&preceding_variant)
            .iter()
            .skip_while(|comment| comment.lines_before() == 0)
            .any(|comment| JsCommentStyle::is_suppression(comment.piece().text()))
    }
    // If this is the first variant, then see if the union has a leading suppression comment.
    else if let Some(union) = list.parent::<TsUnionType>() {
        comments
            .leading_comments(union.syntax())
            .iter()
            .any(|comment| JsCommentStyle::is_suppression(comment.piece().text()))
    } else {
        false
    }
}
