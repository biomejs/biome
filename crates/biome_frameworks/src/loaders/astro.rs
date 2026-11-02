use crate::{
    Activation, ConfigValue, Contribution, Declaration, DeclarationKind, Facet, FrameworkLoader,
    LoaderContext,
};
use biome_glob::Glob;
use std::str::FromStr;

/// Loads [Astro](https://astro.build).
///
/// Astro declares its `astro:*` modules in `astro/client.d.ts`, defines the
/// CSS custom properties of the fonts in `fonts`, and runs the integrations
/// in `integrations`. It also renders the Markdown, MDX, and Markdoc files
/// of its source directory, whose class attributes Biome can't read.
#[derive(Debug)]
pub struct AstroLoader;

/// The files of the source directory that Astro renders without Biome being
/// able to read their class attributes: the Markdown extensions that Astro
/// supports, MDX, and Markdoc.
const CONTENT_FILES: &str = "**/*.{md,markdown,mdown,mkdn,mkd,mdwn,mdx,mdoc}";

impl FrameworkLoader for AstroLoader {
    fn package_name(&self) -> &'static str {
        "astro"
    }

    fn supported_versions(&self) -> &'static str {
        ">=4.0.0"
    }

    fn activation(&self) -> Activation {
        Activation::Dependency {
            config_files: &[
                "astro.config.mjs",
                "astro.config.js",
                "astro.config.ts",
                "astro.config.mts",
                "astro.config.cjs",
                "astro.config.cts",
            ],
        }
    }

    fn load(&self, ctx: &LoaderContext, options: &ConfigValue) -> Vec<Contribution> {
        let mut contributions = Vec::new();

        let client_types = ctx.framework().path.join("client.d.ts");
        contributions.push(if ctx.is_file(&client_types) {
            Contribution::Visit(client_types)
        } else {
            Contribution::Incomplete(Facet::Modules)
        });

        let options = match options.as_call() {
            Some(call)
                if call.specifier.text() == "astro/config"
                    && call.export.text() == "defineConfig" =>
            {
                call.options()
            }
            _ => options.clone(),
        };

        let source_directory = match options.get("srcDir") {
            None => Some(ctx.package_path().join("src")),
            Some(ConfigValue::String { value, .. }) => {
                let directory = value.text().trim_start_matches("./");
                Some(ctx.package_path().join(directory))
            }
            Some(_) => None,
        };
        let has_content_files = source_directory.is_none_or(|directory| {
            Glob::from_str(CONTENT_FILES).is_ok_and(|glob| !ctx.glob(&directory, &glob).is_empty())
        });
        if has_content_files {
            contributions.push(Contribution::Incomplete(Facet::ClassUsages));
        }

        declare_fonts(ctx, options.get("fonts"), &mut contributions);
        if let Some(experimental) = options.get("experimental") {
            declare_fonts(ctx, experimental.get("fonts"), &mut contributions);
        }
        if let Some(integrations) = options.get("integrations") {
            register_integrations(integrations, &mut contributions);
        }

        contributions
    }
}

/// Declares the CSS custom property that the `cssVariable` option of each
/// font defines.
fn declare_fonts(
    ctx: &LoaderContext,
    fonts: Option<&ConfigValue>,
    contributions: &mut Vec<Contribution>,
) {
    let Some(fonts) = fonts else {
        return;
    };
    let Some(fonts) = fonts.as_array() else {
        contributions.push(Contribution::Incomplete(Facet::Styles));
        return;
    };
    for font in fonts {
        match font.get("cssVariable") {
            Some(ConfigValue::String { value, range }) => {
                if let Some(origin) = ctx.origin(*range) {
                    contributions.push(Contribution::Declare(Declaration {
                        kind: DeclarationKind::CssCustomProperty,
                        name: value.clone(),
                        origin,
                    }));
                }
            }
            _ => contributions.push(Contribution::Incomplete(Facet::Styles)),
        }
    }
}

/// Registers the integrations of `integrations`, an array that Astro
/// flattens.
///
/// Integrations that can't be identified, such as the result of a local
/// function, are ignored.
fn register_integrations(integrations: &ConfigValue, contributions: &mut Vec<Contribution>) {
    let Some(integrations) = integrations.as_array() else {
        return;
    };
    for integration in integrations {
        if integration.as_array().is_some() {
            register_integrations(integration, contributions);
        } else if let Some(call) = integration.as_call()
            && let Some(package) = call.package_name()
        {
            contributions.push(Contribution::Integration {
                package: package.to_string().into(),
                options: call.options(),
            });
        }
    }
}
