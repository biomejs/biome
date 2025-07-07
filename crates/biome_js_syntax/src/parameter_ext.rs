use crate::{
    AnyJsBindingPattern, AnyJsConstructorParameter, AnyJsFormalParameter, AnyJsParameter,
    JsConstructorParameterList, JsConstructorParameters, JsDecoratorList, JsLanguage,
    JsParameterList, JsParameters, TsTypeAnnotation,
};
use biome_rowan::{
    AstNodeList, AstSeparatedList, AstSeparatedListNodesIterator, SyntaxResult, declare_node_union,
};

/// An enumeration representing different types of JavaScript/TypeScript parameter lists.
///
/// This enum can represent a regular JavaScript/TypeScript parameter list (i.e., for functions)
/// or a JavaScript/TypeScript constructor parameter list (i.e., for class constructors).
///
/// # Examples
///
/// ```
/// use biome_js_factory::make;
/// use biome_js_syntax::{AnyJsBinding, AnyJsBindingPattern, AnyJsConstructorParameter, AnyJsFormalParameter, AnyJsParameter};
/// use biome_js_syntax::parameter_ext::AnyJsParameterList;
///
/// // Create a function parameter list
/// let parameter_list = make::js_parameter_list(
///     Some(AnyJsParameter::AnyJsFormalParameter(
///         AnyJsFormalParameter::JsFormalParameter(
///             make::js_formal_parameter(
///                 make::js_decorator_list([]),
///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
///                     make::js_identifier_binding(make::ident("params")),
///                 )),
///             )
///             .build(),
///         ),
///     )),
///     None,
/// );
/// let function_params = AnyJsParameterList::JsParameterList(parameter_list);
///
/// // Create a constructor parameter list
/// let constructor_parameter_list = make::js_constructor_parameter_list(
///     Some(AnyJsConstructorParameter::AnyJsFormalParameter(
///         AnyJsFormalParameter::JsFormalParameter(
///             make::js_formal_parameter(
///                 make::js_decorator_list([]),
///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
///                     make::js_identifier_binding(make::ident("params")),
///                 )),
///             )
///             .build(),
///         ),
///     )),
///     None,
/// );
///
/// let constructor_params = AnyJsParameterList::JsConstructorParameterList(constructor_parameter_list);
/// ```
///
/// # Variants
///
/// * `JsParameterList` - A list of parameters for a JavaScript function.
/// * `JsConstructorParameterList` - A list of parameters for a JavaScript constructor.
#[derive(Debug)]
pub enum AnyJsParameterList {
    JsParameterList(JsParameterList),
    JsConstructorParameterList(JsConstructorParameterList),
}

impl From<JsParameterList> for AnyJsParameterList {
    fn from(list: JsParameterList) -> Self {
        Self::JsParameterList(list)
    }
}

impl From<JsConstructorParameterList> for AnyJsParameterList {
    fn from(list: JsConstructorParameterList) -> Self {
        Self::JsConstructorParameterList(list)
    }
}

impl AnyJsParameterList {
    ///
    /// This method allows to get the length of a parameter list, regardless
    /// of whether it's a standard parameter list or a constructor parameter list.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::parameter_ext::AnyJsParameterList;
    /// use biome_js_syntax::{
    ///     AnyJsBinding, AnyJsBindingPattern, AnyJsConstructorParameter, AnyJsFormalParameter,
    ///     AnyJsParameter, T,
    /// };
    ///
    /// let parameter_list = make::js_parameter_list(
    ///     Some(AnyJsParameter::AnyJsFormalParameter(
    ///         AnyJsFormalParameter::JsFormalParameter(
    ///             make::js_formal_parameter(
    ///                 make::js_decorator_list([]),
    ///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
    ///                     make::js_identifier_binding(make::ident("params")),
    ///                 )),
    ///             )
    ///             .build(),
    ///         ),
    ///     )),
    ///     None,
    /// );
    ///
    /// let params = AnyJsParameterList::JsParameterList(parameter_list);
    /// assert_eq!(params.len(), 1);
    ///
    /// let constructor_parameter_list = make::js_constructor_parameter_list(
    ///     Some(AnyJsConstructorParameter::AnyJsFormalParameter(
    ///         AnyJsFormalParameter::JsFormalParameter(
    ///             make::js_formal_parameter(
    ///                 make::js_decorator_list([]),
    ///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
    ///                     make::js_identifier_binding(make::ident("params")),
    ///                 )),
    ///             )
    ///             .build(),
    ///         ),
    ///     )),
    ///     None,
    /// );
    ///
    /// let params = AnyJsParameterList::JsConstructorParameterList(constructor_parameter_list);
    /// assert_eq!(params.len(), 1);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns the length of the parameter list.
    pub fn len(&self) -> usize {
        match self {
            Self::JsParameterList(parameters) => parameters.len(),
            Self::JsConstructorParameterList(parameters) => parameters.len(),
        }
    }

    ///
    /// This method checks if a parameter list is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::parameter_ext::AnyJsParameterList;
    /// use biome_js_syntax::{
    ///     AnyJsBinding, AnyJsBindingPattern, AnyJsConstructorParameter, AnyJsFormalParameter,
    ///     AnyJsParameter, T,
    /// };
    ///
    /// let parameter_list = make::js_parameter_list(
    ///     Some(AnyJsParameter::AnyJsFormalParameter(
    ///         AnyJsFormalParameter::JsFormalParameter(
    ///             make::js_formal_parameter(
    ///                 make::js_decorator_list([]),
    ///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
    ///                     make::js_identifier_binding(make::ident("params")),
    ///                 )),
    ///             )
    ///             .build(),
    ///         ),
    ///     )),
    ///     None,
    /// );
    ///
    /// let params = AnyJsParameterList::JsParameterList(parameter_list);
    /// assert_eq!(params.is_empty(), false);
    ///
    /// let constructor_parameter_list = make::js_constructor_parameter_list(
    ///     None,
    ///     None,
    /// );
    ///
    /// let params = AnyJsParameterList::JsConstructorParameterList(constructor_parameter_list);
    /// assert!(params.is_empty());
    /// ```
    ///
    /// # Returns
    ///
    /// Returns `true` if the parameter list contains no parameters, false otherwise.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::JsParameterList(parameters) => parameters.is_empty(),
            Self::JsConstructorParameterList(parameters) => parameters.is_empty(),
        }
    }

    ///
    /// This method allows to get the first parameter in the parameter list.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::parameter_ext::{AnyJsParameterList, AnyParameter};
    /// use biome_js_syntax::{
    ///     AnyJsBinding, AnyJsBindingPattern, AnyJsConstructorParameter, AnyJsFormalParameter,
    ///     AnyJsParameter, T,
    /// };
    /// use biome_rowan::SyntaxResult;
    ///
    /// let parameter_list = make::js_parameter_list(
    ///     Some(AnyJsParameter::AnyJsFormalParameter(
    ///         AnyJsFormalParameter::JsFormalParameter(
    ///             make::js_formal_parameter(
    ///                 make::js_decorator_list([]),
    ///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
    ///                     make::js_identifier_binding(make::ident("param1")),
    ///                 )),
    ///             )
    ///             .build(),
    ///         ),
    ///     )),
    ///     None,
    /// );
    ///
    /// let params = AnyJsParameterList::JsParameterList(parameter_list);
    /// let first_param = params.first().unwrap();
    /// assert_eq!(first_param.is_ok(), true);
    ///
    /// let empty_parameter_list = make::js_constructor_parameter_list(None, None);
    /// let empty_params = AnyJsParameterList::JsConstructorParameterList(empty_parameter_list);
    /// assert!(empty_params.first().is_none());
    /// ```
    ///
    /// # Returns
    ///
    /// Returns the first parameter in the parameter list if it exists.
    pub fn first(&self) -> Option<SyntaxResult<AnyParameter>> {
        Some(match self {
            Self::JsParameterList(parameters) => {
                parameters.first()?.map(|parameter| parameter.into())
            }
            Self::JsConstructorParameterList(parameters) => {
                parameters.first()?.map(|parameter| parameter.into())
            }
        })
    }

    ///
    /// This method allows you to iterate over the parameters in a `JsParameterList` or a `JsConstructorParameterList`,
    /// depending on the variant of the `AnyJsParameterList` enum.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::parameter_ext::AnyJsParameterList;
    /// use biome_js_syntax::{
    ///     AnyJsBinding, AnyJsBindingPattern, AnyJsConstructorParameter, AnyJsFormalParameter,
    ///     AnyJsParameter, T,
    /// };
    ///
    /// let parameter_list = make::js_parameter_list(
    ///     Some(AnyJsParameter::AnyJsFormalParameter(
    ///         AnyJsFormalParameter::JsFormalParameter(
    ///             make::js_formal_parameter(
    ///                 make::js_decorator_list([]),
    ///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
    ///                     make::js_identifier_binding(make::ident("param1")),
    ///                 )),
    ///             )
    ///             .build(),
    ///         ),
    ///     )),
    ///     None,
    /// );
    ///
    /// let params = AnyJsParameterList::JsParameterList(parameter_list);
    /// let mut iter = params.iter();
    ///
    /// assert_eq!(iter.next().is_some(), true);
    /// assert_eq!(iter.next().is_none(), true);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns an iterator over the parameters in the list.
    ///
    pub fn iter(&self) -> AnyJsParameterListNodeIter {
        match self {
            Self::JsParameterList(list) => AnyJsParameterListNodeIter::JsParameterList(list.iter()),
            Self::JsConstructorParameterList(list) => {
                AnyJsParameterListNodeIter::JsConstructorParameterList(list.iter())
            }
        }
    }

    ///
    /// This method allows to get the last parameter in the parameter list.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::parameter_ext::AnyJsParameterList;
    /// use biome_js_syntax::{
    ///     AnyJsBinding, AnyJsBindingPattern, AnyJsConstructorParameter, AnyJsFormalParameter,
    ///     AnyJsParameter, T,
    /// };
    ///
    /// let parameter_list = make::js_parameter_list(
    ///     Some(AnyJsParameter::AnyJsFormalParameter(
    ///         AnyJsFormalParameter::JsFormalParameter(
    ///             make::js_formal_parameter(
    ///                 make::js_decorator_list([]),
    ///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
    ///                     make::js_identifier_binding(make::ident("param1")),
    ///                 )),
    ///             )
    ///             .build(),
    ///         ),
    ///     )),
    ///     None,
    /// );
    ///
    /// let params = AnyJsParameterList::JsParameterList(parameter_list);
    /// let last_param = params.last().unwrap();
    /// assert_eq!(last_param.is_ok(), true);
    ///
    /// let empty_parameter_list = make::js_parameter_list(None, None);
    /// let empty_params = AnyJsParameterList::JsParameterList(empty_parameter_list);
    /// assert!(empty_params.last().is_none());
    /// ```
    ///
    /// # Returns
    ///
    /// Returns the last parameter in the parameter list if it exists.
    ///
    pub fn last(&self) -> Option<SyntaxResult<AnyParameter>> {
        Some(match self {
            Self::JsParameterList(parameters) => {
                parameters.last()?.map(|parameter| parameter.into())
            }
            Self::JsConstructorParameterList(parameters) => {
                parameters.last()?.map(|parameter| parameter.into())
            }
        })
    }

    ///
    /// This method checks if any parameters in the given list are decorated.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::parameter_ext::{AnyJsParameterList, AnyParameter};
    /// use biome_js_syntax::{
    ///     AnyJsBinding, AnyJsBindingPattern, AnyJsConstructorParameter, AnyJsDecorator,
    ///     AnyJsFormalParameter, AnyJsParameter, T,
    /// };
    /// use biome_rowan::SyntaxResult;
    ///
    /// let parameter_list = make::js_parameter_list(
    ///     Some(AnyJsParameter::AnyJsFormalParameter(
    ///         AnyJsFormalParameter::JsFormalParameter(
    ///             make::js_formal_parameter(
    ///                 make::js_decorator_list([]),
    ///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
    ///                     make::js_identifier_binding(make::ident("param1")),
    ///                 )),
    ///             )
    ///             .build(),
    ///         ),
    ///     )),
    ///     None,
    /// );
    ///
    /// let params = AnyJsParameterList::JsParameterList(parameter_list);
    /// let has_any_decorated_parameter = params.has_any_decorated_parameter();
    /// assert_eq!(has_any_decorated_parameter, false);
    ///
    /// let decorator = make::js_decorator(
    ///     make::token(T![@]),
    ///     AnyJsDecorator::JsIdentifierExpression(make::js_identifier_expression(
    ///         make::js_reference_identifier(make::ident("decorator")),
    ///     )),
    /// );
    /// let parameter_list = make::js_parameter_list(
    ///     Some(AnyJsParameter::AnyJsFormalParameter(
    ///         AnyJsFormalParameter::JsFormalParameter(
    ///             make::js_formal_parameter(
    ///                 make::js_decorator_list(Some(decorator)),
    ///                 AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
    ///                     make::js_identifier_binding(make::ident("param1")),
    ///                 )),
    ///             )
    ///             .build(),
    ///         ),
    ///     )),
    ///     None,
    /// );
    ///
    /// let params = AnyJsParameterList::JsParameterList(parameter_list);
    /// let has_any_decorated_parameter = params.has_any_decorated_parameter();
    /// assert_eq!(has_any_decorated_parameter, true);
    /// ```
    ///
    /// # Returns
    ///
    /// Returns `true` if the list contains any decorated parameters.
    ///
    pub fn has_any_decorated_parameter(&self) -> bool {
        self.iter().any(|parameter| {
            parameter.is_ok_and(|parameter| match parameter {
                AnyParameter::AnyJsConstructorParameter(parameter) => parameter.has_any_decorator(),
                AnyParameter::AnyJsParameter(parameter) => parameter.has_any_decorator(),
            })
        })
    }
}

/// An iterator over the parameters in an `AnyJsParameterList`.
///
/// This iterator can traverse a regular JavaScript/TypeScript parameter list (i.e., for functions)
/// or a JavaScript/TypeScript constructor parameter list (i.e., for class constructors), depending
/// on the variant of the `AnyJsParameterListNodeIter` enum.
pub enum AnyJsParameterListNodeIter {
    JsParameterList(AstSeparatedListNodesIterator<JsLanguage, AnyJsParameter>),
    JsConstructorParameterList(
        AstSeparatedListNodesIterator<JsLanguage, AnyJsConstructorParameter>,
    ),
}

impl Iterator for AnyJsParameterListNodeIter {
    type Item = SyntaxResult<AnyParameter>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self {
            Self::JsParameterList(inner) => inner.next()?.map(AnyParameter::from),
            Self::JsConstructorParameterList(inner) => inner.next()?.map(AnyParameter::from),
        })
    }
}

declare_node_union! {
    /// The `AnyParameter` union can represent either a standard JavaScript/TypeScript parameter
    /// or a JavaScript/TypeScript constructor parameter. This is useful in contexts where a
    /// function could accept either type of parameter.
    pub AnyParameter = AnyJsConstructorParameter | AnyJsParameter
}

impl AnyParameter {
    pub fn binding(&self) -> Option<AnyJsBindingPattern> {
        match self {
            Self::AnyJsConstructorParameter(parameter) => match parameter {
                AnyJsConstructorParameter::AnyJsFormalParameter(parameter) => {
                    parameter.as_js_formal_parameter()?.binding().ok()
                }
                AnyJsConstructorParameter::JsRestParameter(parameter) => parameter.binding().ok(),
                AnyJsConstructorParameter::TsPropertyParameter(parameter) => parameter
                    .formal_parameter()
                    .ok()?
                    .as_js_formal_parameter()?
                    .binding()
                    .ok(),
            },
            Self::AnyJsParameter(parameter) => match parameter {
                AnyJsParameter::AnyJsFormalParameter(parameter) => {
                    parameter.as_js_formal_parameter()?.binding().ok()
                }
                AnyJsParameter::JsRestParameter(parameter) => parameter.binding().ok(),
                AnyJsParameter::TsThisParameter(_) => None,
            },
        }
    }

    /// Returns type annotation of the parameter if any.
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        match self {
            Self::AnyJsConstructorParameter(parameter) => parameter.type_annotation(),
            Self::AnyJsParameter(parameter) => parameter.type_annotation(),
        }
    }
}

declare_node_union! {
    /// The `AnyJsParameters` union can represent either a standard JavaScript/TypeScript parameters
    /// or a JavaScript/TypeScript constructor parameters. This is useful in contexts where a
    /// function could accept either type of parameters.
    pub AnyJsParameters = JsParameters | JsConstructorParameters
}

impl AnyJsConstructorParameter {
    /// Returns the list of decorators of the parameter if the parameter is decorated.
    pub fn decorators(&self) -> Option<JsDecoratorList> {
        match self {
            Self::AnyJsFormalParameter(parameter) => parameter.decorators(),
            Self::JsRestParameter(parameter) => Some(parameter.decorators()),
            Self::TsPropertyParameter(parameter) => Some(parameter.decorators()),
        }
    }

    /// Returns `true` if any parameter in the given list is decorated.
    pub fn has_any_decorator(&self) -> bool {
        self.decorators()
            .is_some_and(|decorators| !decorators.is_empty())
    }

    /// Returns the type annotation of the parameter if any.
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        match self {
            Self::AnyJsFormalParameter(parameter) => parameter.type_annotation(),
            Self::JsRestParameter(parameter) => parameter.type_annotation(),
            Self::TsPropertyParameter(parameter) => {
                parameter.formal_parameter().ok()?.type_annotation()
            }
        }
    }
}

impl AnyJsParameter {
    /// Returns the list of decorators of the parameter if the parameter is decorated.
    pub fn decorators(&self) -> Option<JsDecoratorList> {
        match self {
            Self::AnyJsFormalParameter(parameter) => parameter.decorators(),
            Self::JsRestParameter(parameter) => Some(parameter.decorators()),
            Self::TsThisParameter(_) => None,
        }
    }

    /// Returns `true` if any parameter in the given list is decorated.
    pub fn has_any_decorator(&self) -> bool {
        self.decorators()
            .is_some_and(|decorators| !decorators.is_empty())
    }

    /// Returns the type annotation of the parameter if any.
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        match self {
            Self::AnyJsFormalParameter(parameter) => parameter.type_annotation(),
            Self::JsRestParameter(parameter) => parameter.type_annotation(),
            Self::TsThisParameter(parameter) => parameter.type_annotation(),
        }
    }
}

impl AnyJsFormalParameter {
    /// Returns the list of decorators of the parameter if the parameter is decorated.
    pub fn decorators(&self) -> Option<JsDecoratorList> {
        match self {
            Self::JsBogusParameter(_) | Self::JsMetavariable(_) => None,
            Self::JsFormalParameter(parameter) => Some(parameter.decorators()),
        }
    }

    /// Returns the type annotation of the parameter if any.
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        match self {
            Self::JsBogusParameter(_) | Self::JsMetavariable(_) => None,
            Self::JsFormalParameter(parameter) => parameter.type_annotation(),
        }
    }
}
