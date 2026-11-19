//! Queries over the pages of Next.js applications.

use crate::ModuleDb;
use biome_resolver::ResolverPathData;

/// Returns the pages of the Next.js app in the directory of `project`, which
/// contains `app`/`pages` or `src/app`/`src/pages`.
///
/// Reads only module paths, not their kind, which changes on every edit.
#[salsa::tracked(returns(deref))]
pub fn next_pages(db: &dyn ModuleDb, project: ResolverPathData) -> Vec<NextPage> {
    let project_path = project.path(db);

    let mut pages = Vec::new();
    db.for_each_module(&mut |module| {
        let path = module.path(db);
        if path.starts_with(project_path)
            && let Some(extension) = path.extension()
            && PAGE_EXTENSIONS.contains(&extension)
            && let Some(file_stem) = path.file_stem()
            && let Some(directory) = path.parent()
        {
            for router in directory.ancestors() {
                if let Some(router_name) = router.file_name()
                    && let Some(parent) = router.parent()
                    && let Ok(directories) = directory.strip_prefix(router)
                {
                    let directories = directories.components().map(|component| component.as_str());

                    let route = match router_name {
                        "app" => {
                            // Private folders and intercepted routes don't define
                            // URLs of their own.
                            if file_stem != "page"
                                || directories.clone().any(|directory| {
                                    directory.starts_with('_') || directory.starts_with("(.")
                                })
                            {
                                continue;
                            }
                            directories
                                .filter(|directory| {
                                    let is_route_group =
                                        directory.starts_with('(') && directory.ends_with(')');
                                    !is_route_group && !directory.starts_with('@')
                                })
                                .map(Box::from)
                                .collect()
                        }
                        "pages" => {
                            let route =
                                directories.chain((file_stem != "index").then_some(file_stem));
                            let mut segments = route.clone();
                            let is_special_file = matches!(
                                (segments.next(), segments.next()),
                                (Some("api"), _) | (Some("_app" | "_document" | "_error"), None)
                            );
                            if is_special_file {
                                continue;
                            }
                            route.map(Box::from).collect()
                        }
                        _ => continue,
                    };

                    let page_project_path = if parent.file_name() == Some("src") {
                        parent.parent().unwrap_or(parent)
                    } else {
                        parent
                    };
                    // A nested project keeps its own pages.
                    if page_project_path == project_path {
                        pages.push(NextPage { route });
                    }
                }
            }
        }
    });
    pages
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NextPage {
    /// The URL segments of the page, without route groups and parallel route
    /// slots. Dynamic segments are kept as written, such as `[slug]`,
    /// `[...slug]`, or `[[...slug]]`.
    route: Box<[Box<str>]>,
}

impl NextPage {
    /// Returns whether the URL `path`, without query and fragment, leads to
    /// this page.
    pub fn matches(&self, path: &str) -> bool {
        // Like `@next/next/no-html-link-for-pages`, dynamic segments don't
        // match file names such as `report.pdf`, which are usually served
        // from `public`.
        let is_dynamic_value = |segment: &str| !segment.contains('.');
        let mut segments = path.split('/').filter(|segment| !segment.is_empty());
        for route_segment in &self.route {
            if route_segment.starts_with("[[...") {
                return segments.all(is_dynamic_value);
            }
            if route_segment.starts_with("[...") {
                return segments.next().is_some_and(is_dynamic_value)
                    && segments.all(is_dynamic_value);
            }
            let Some(segment) = segments.next() else {
                return false;
            };
            let segment_matches = if route_segment.starts_with('[') {
                is_dynamic_value(segment)
            } else {
                route_segment.as_ref() == segment
            };
            if !segment_matches {
                return false;
            }
        }
        segments.next().is_none()
    }
}

/// The default `pageExtensions` of Next.js.
const PAGE_EXTENSIONS: [&str; 4] = ["tsx", "ts", "jsx", "js"];

#[cfg(test)]
mod tests {
    use super::*;

    fn page(route: &[&str]) -> NextPage {
        NextPage {
            route: route.iter().copied().map(Box::from).collect(),
        }
    }

    #[test]
    fn matches_root() {
        assert!(page(&[]).matches("/"));
        assert!(page(&[]).matches(""));
        assert!(!page(&[]).matches("/about"));
    }

    #[test]
    fn matches_static_segments() {
        let about_team = page(&["about", "team"]);
        assert!(about_team.matches("/about/team"));
        assert!(about_team.matches("/about/team/"));
        assert!(about_team.matches("//about//team"));
        assert!(!about_team.matches("/about"));
        assert!(!about_team.matches("/about/team/history"));
        assert!(!about_team.matches("/about/careers"));
    }

    #[test]
    fn matches_dynamic_segment() {
        let post = page(&["blog", "[slug]"]);
        assert!(post.matches("/blog/hello-world"));
        assert!(!post.matches("/blog"));
        assert!(!post.matches("/blog/hello/world"));
        assert!(!post.matches("/blog/report.pdf"));
    }

    #[test]
    fn matches_catch_all_segment() {
        let docs = page(&["docs", "[...path]"]);
        assert!(docs.matches("/docs/getting-started"));
        assert!(docs.matches("/docs/getting-started/installation"));
        assert!(!docs.matches("/docs"));
        assert!(!docs.matches("/docs/guide/report.pdf"));
    }

    #[test]
    fn matches_optional_catch_all_segment() {
        let shop = page(&["shop", "[[...filters]]"]);
        assert!(shop.matches("/shop"));
        assert!(shop.matches("/shop/shoes/red"));
        assert!(!shop.matches("/shop/catalog.pdf"));
        assert!(!shop.matches("/store"));
    }
}
