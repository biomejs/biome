use crate::{registry::RuleRoot, FromServices, Queryable, Rule, RuleKey, ServiceBag};
use biome_diagnostics::{Error, Result};
use std::ops::Deref;
use std::path::Path;

type RuleQueryResult<R> = <<R as Rule>::Query as Queryable>::Output;
type RuleServiceBag<R> = <<R as Rule>::Query as Queryable>::Services;

pub struct RuleContext<'a, R>
where
    R: ?Sized + Rule,
{
    query_result: &'a RuleQueryResult<R>,
    root: &'a RuleRoot<R>,
    bag: &'a ServiceBag,
    services: RuleServiceBag<R>,
    globals: &'a [&'a str],
    file_path: &'a Path,
    options: &'a R::Options,
}

impl<'a, R> RuleContext<'a, R>
where
    R: Rule + Sized + 'static,
{
    pub fn new(
        query_result: &'a RuleQueryResult<R>,
        root: &'a RuleRoot<R>,
        services: &'a ServiceBag,
        globals: &'a [&'a str],
        file_path: &'a Path,
        options: &'a R::Options,
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
        })
    }

    pub fn query(&self) -> &RuleQueryResult<R> {
        self.query_result
    }

    /// Returns a clone of the AST root
    pub fn root(&self) -> RuleRoot<R> {
        self.root.clone()
    }

    /// It retrieves the options that belong to a rule, if they exist.
    ///
    /// In order to retrieve a typed data structure, you have to create a deserializable
    /// data structure and define it inside the generic type `type Options` of the [Rule]
    ///
    /// ## Examples
    ///
    /// ```rust,ignore
    /// use biome_analyze::{declare_rule, Rule, RuleCategory, RuleMeta, RuleMetadata};
    /// use biome_analyze::context::RuleContext;
    /// use serde::Deserialize;
    /// declare_rule! {
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
    pub fn file_path(&self) -> &Path {
        self.file_path
    }
}

impl<'a, R> Deref for RuleContext<'a, R>
where
    R: Rule,
{
    type Target = RuleServiceBag<R>;

    fn deref(&self) -> &Self::Target {
        &self.services
    }
}
