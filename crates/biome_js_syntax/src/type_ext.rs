use std::iter;

use crate::AnyTsType;

impl AnyTsType {
    /// Try to extract non `TsParenthesizedType` from `AnyTsType`
    pub fn omit_parentheses(self) -> AnyTsType {
        let first = self.as_ts_parenthesized_type().and_then(|x| x.ty().ok());
        iter::successors(first, |x| {
            let parenthesized = x.as_ts_parenthesized_type()?;
            parenthesized.ty().ok()
        })
        .last()
        .unwrap_or(self)
    }

    /// Returns `true` if `self` is a literal type.
    ///
    /// ### Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    /// use biome_js_syntax::AnyTsType;
    ///
    /// let boolean_literal = make::ts_boolean_literal_type(make::token(T![true]));
    /// let bigint_literal = make::ts_bigint_literal_type(make::js_number_literal("1n")).build();
    /// let null_literal = make::ts_null_literal_type(make::token(T![null]));
    /// let number_literal = make::ts_number_literal_type(make::js_number_literal("1")).build();
    /// let string_literal = make::ts_string_literal_type(make::js_string_literal("s"));
    /// let undefined = make::ts_undefined_type(make::token(T![undefined]));
    ///
    /// assert!(AnyTsType::TsBooleanLiteralType(boolean_literal).is_literal_type());
    /// assert!(AnyTsType::TsBigintLiteralType(bigint_literal).is_literal_type());
    /// assert!(AnyTsType::TsNullLiteralType(null_literal).is_literal_type());
    /// assert!(AnyTsType::TsNumberLiteralType(number_literal).is_literal_type());
    /// assert!(AnyTsType::TsStringLiteralType(string_literal).is_literal_type());
    /// assert!(AnyTsType::TsUndefinedType(undefined).is_literal_type());
    /// ```
    pub fn is_literal_type(&self) -> bool {
        matches!(
            self,
            AnyTsType::TsBooleanLiteralType(_)
                | AnyTsType::TsBigintLiteralType(_)
                | AnyTsType::TsNullLiteralType(_)
                | AnyTsType::TsNumberLiteralType(_)
                | AnyTsType::TsStringLiteralType(_)
                | AnyTsType::TsUndefinedType(_)
        )
    }

    /// Returns `true` if `self` is a primitive type.
    ///
    /// ### Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::T;
    /// use biome_js_syntax::AnyTsType;
    ///
    /// let boolean = make::ts_boolean_type(make::token(T![boolean]));
    /// let bigint = make::ts_bigint_type(make::token(T![bigint]));
    /// let number = make::ts_number_type(make::token(T![number]));
    /// let string = make::ts_string_type(make::token(T![string]));
    ///
    /// assert!(AnyTsType::TsBooleanType(boolean).is_primitive_type());
    /// assert!(AnyTsType::TsBigintType(bigint).is_primitive_type());
    /// assert!(AnyTsType::TsNumberType(number).is_primitive_type());
    /// assert!(AnyTsType::TsStringType(string).is_primitive_type());
    /// ```
    pub fn is_primitive_type(&self) -> bool {
        matches!(
            self,
            AnyTsType::TsBooleanType(_)
                | AnyTsType::TsBigintType(_)
                | AnyTsType::TsNumberType(_)
                | AnyTsType::TsStringType(_)
        )
    }
}
