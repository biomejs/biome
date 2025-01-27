use crate::options::{JsxRuntime, PreferredQuote};
use crate::{registry::RuleRoot, FromServices, Queryable, Rule, RuleKey, ServiceBag};
use crate::{GroupCategory, RuleCategory, RuleGroup, RuleMetadata};
use biome_diagnostics::{Error, Result};
use camino::Utf8Path;
use std::ops::Deref;

type RuleQueryResult<R> = <<R as Rule>::Query as Queryable>::Output;
type RuleServiceBag<R> = <<R as Rule>::Query as Queryable>::Services;

pub struct RuleContext<'a, R: Rule> {
    query_result: &'a RuleQueryResult<R>,
    root: &'a RuleRoot<R>,
    bag: &'a ServiceBag,
    services: RuleServiceBag<R>,
    globals: &'a [&'a str],
    file_path: &'a Utf8Path,
    options: &'a R::Options,
    preferred_quote: &'a PreferredQuote,
    preferred_jsx_quote: &'a PreferredQuote,
    jsx_runtime: Option<JsxRuntime>,
}

impl<'a, R> RuleContext<'a, R>
where
    R: Rule + Sized + 'static,
{
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        query_result: &'a RuleQueryResult<R>,
        root: &'a RuleRoot<R>,
        services: &'a ServiceBag,
        globals: &'a [&'a str],
        file_path: &'a Utf8Path,
        options: &'a R::Options,
        preferred_quote: &'a PreferredQuote,
        preferred_jsx_quote: &'a PreferredQuote,
        jsx_runtime: Option<JsxRuntime>,
    ) -> Result<Self, Error> {
        let rule_key = RuleKey::rule::<R>();
        Ok(Self {
            query_result,
            root,
            bag: services,
            services: FromServices::from_services(&rule_key, services)?,
            globals,
            file_path,
            options,
            preferred_quote,
            preferred_jsx_quote,
            jsx_runtime,
        })
    }

    pub fn query(&self) -> &RuleQueryResult<R> {
        self.query_result
    }

    /// Returns the group that belongs to the current rule
    pub fn group(&self) -> &'static str {
        <R::Group as RuleGroup>::NAME
    }

    /// Returns the category that belongs to the current rule
    pub fn category(&self) -> RuleCategory {
        <<R::Group as RuleGroup>::Category as GroupCategory>::CATEGORY
    }

    /// Returns a clone of the AST root
    pub fn root(&self) -> RuleRoot<R> {
        self.root.clone()
    }

    /// Returns the metadata of the rule
    ///
    /// The metadata contains information about the rule, such as the name, version, language, and whether it is recommended.
    ///
    /// ## Examples
    /// ```rust,ignore
    /// declare_lint_rule! {
    ///     /// Some doc
    ///     pub(crate) Foo {
    ///         version: "0.0.0",
    ///         name: "foo",
    ///         language: "js",
    ///         recommended: true,
    ///     }
    /// }
    ///
    /// impl Rule for Foo {
    ///     const CATEGORY: RuleCategory = RuleCategory::Lint;
    ///     type Query = ();
    ///     type State = ();
    ///     type Signals = ();
    ///     type Options = ();
    ///
    ///     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    ///         assert_eq!(ctx.metadata().name, "foo");
    ///     }
    /// }
    /// ```
    pub fn metadata(&self) -> &RuleMetadata {
        &R::METADATA
    }

    /// It retrieves the options that belong to a rule, if they exist.
    ///
    /// In order to retrieve a typed data structure, you have to create a deserializable
    /// data structure and define it inside the generic type `type Options` of the [Rule]
    ///
    /// ## Examples
    ///
    /// ```rust,ignore
    /// use biome_analyze::{declare_lint_rule, Rule, RuleCategory, RuleMeta, RuleMetadata};
    /// use biome_analyze::context::RuleContext;
    /// use serde::Deserialize;
    /// declare_lint_rule! {
    ///     /// Some doc
    ///     pub(crate) Name {
    ///         version: "0.0.0",
    ///         name: "name",
    ///         recommended: true,
    ///     }
    /// }
    ///
    /// #[derive(Deserialize)]
    /// struct RuleOptions {}
    ///
    /// impl Rule for Name {
    ///     const CATEGORY: RuleCategory = RuleCategory::Lint;
    ///     type Query = ();
    ///     type State = ();
    ///     type Signals = ();
    ///     type Options = RuleOptions;
    ///
    ///     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    ///         if let Some(options) = ctx.options() {
    ///             // do something with the options now
    ///         }
    ///     }
    /// }
    /// ```
    pub fn options(&self) -> &R::Options {
        self.options
    }

    /// Returns the JSX runtime in use.
    pub fn jsx_runtime(&self) -> JsxRuntime {
        self.jsx_runtime.expect("jsx_runtime should be provided")
    }

    /// Checks whether the provided text belongs to globals
    pub fn is_global(&self, text: &str) -> bool {
        self.globals.contains(&text)
    }

    /// Returns the source type of the current file
    pub fn source_type<T: 'static>(&self) -> &T {
        self.bag
            .get_service::<T>()
            .expect("Source type is not registered")
    }

    /// The file path of the current file
    pub fn file_path(&self) -> &Utf8Path {
        self.file_path
    }

    /// Returns the preferred quote that should be used when providing code actions
    pub fn as_preferred_quote(&self) -> &PreferredQuote {
        self.preferred_quote
    }

    /// Returns the preferred JSX quote that should be used when providing code actions
    pub fn as_preferred_jsx_quote(&self) -> &PreferredQuote {
        self.preferred_jsx_quote
    }

    /// Attempts to retrieve a service from the current context
    ///
    /// ```no_test
    /// let aria_services = ctx.get_service::<AriaServices>().expect("To have the service available");
    /// ```
    pub fn get_service<T: 'static>(&self) -> Option<&T> {
        self.bag.get_service::<T>()
    }
}

impl<R> Deref for RuleContext<'_, R>
where
    R: Rule,
{
    type Target = RuleServiceBag<R>;

    fn deref(&self) -> &Self::Target {
        &self.services
    }
}
