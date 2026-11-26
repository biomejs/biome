use super::visit_framework_files;
use crate::{
    Activation, ConfigValue, Contribution, Facet, FrameworkLoader, LoaderContext, ResolutionKind,
};

/// Loads [Starlight](https://starlight.astro.build), an Astro integration.
///
/// Every page of Starlight uses its stylesheets, the stylesheets of
/// `customCss`, and its components. Starlight runs the plugins of `plugins`.
#[derive(Debug)]
pub struct StarlightLoader;

impl FrameworkLoader for StarlightLoader {
    fn package_name(&self) -> &'static str {
        "@astrojs/starlight"
    }

    fn supported_versions(&self) -> &'static str {
        ">=0.30.0"
    }

    fn activation(&self) -> Activation {
        Activation::Integration
    }

    fn load(&self, ctx: &LoaderContext, options: &ConfigValue) -> Vec<Contribution> {
        let mut contributions = Vec::new();

        visit_framework_files(
            ctx,
            &["dist/style/*.css", "style/*.css"],
            Facet::Styles,
            &mut contributions,
        );
        visit_framework_files(
            ctx,
            &["dist/**/*.astro", "components/**/*.astro"],
            Facet::ClassUsages,
            &mut contributions,
        );

        match options.get("customCss") {
            None => {}
            Some(ConfigValue::Array(stylesheets)) => {
                for stylesheet in stylesheets {
                    // Like Starlight, a path that starts with `.` is relative
                    // to the project, and other values name packages.
                    let path = stylesheet
                        .as_str()
                        .and_then(|stylesheet| ctx.resolve(stylesheet, ResolutionKind::Stylesheet));
                    contributions.push(match path {
                        Some(path) => Contribution::Visit(path),
                        None => Contribution::Incomplete(Facet::Styles),
                    });
                }
            }
            Some(_) => contributions.push(Contribution::Incomplete(Facet::Styles)),
        }

        if let Some(plugins) = options.get("plugins").and_then(ConfigValue::as_array) {
            for plugin in plugins {
                if let Some(call) = plugin.as_call()
                    && let Some(package) = call.package_name()
                {
                    contributions.push(Contribution::Integration {
                        package: package.to_string().into(),
                        options: call.options(),
                    });
                }
            }
        }

        contributions
    }
}
