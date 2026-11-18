//! Queries over the pages of Next.js applications.

use crate::{ModuleDb, ModuleGraphGeneration};
use camino::Utf8PathBuf;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NextPage {
    /// The directory that contains `app`/`pages`, or `src/app`/`src/pages`.
    pub project_path: Utf8PathBuf,
    /// The URL segments of the page, without route groups and parallel route
    /// slots. Dynamic segments are kept as written, such as `[slug]`,
    /// `[...slug]`, or `[[...slug]]`.
    route: Box<[Box<str>]>,
}

impl NextPage {
    /// Returns whether the URL `path`, without query and fragment, leads to
    /// this page.
    pub fn matches(&self, path: &str) -> bool {
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

/// Returns the pages of all projects: rules keep those of their own project.
///
/// Reads only module paths, not their kind, which changes on every edit.
#[salsa::tracked(returns(deref))]
pub fn next_pages(db: &dyn ModuleDb, generation: ModuleGraphGeneration) -> Vec<NextPage> {
    let _ = generation.value(db);

    let mut pages = Vec::new();
    db.for_each_module(&mut |module| {
        let path = module.path(db);
        if !path
            .extension()
            .is_some_and(|extension| PAGE_EXTENSIONS.contains(&extension))
        {
            return;
        }
        let (Some(file_stem), Some(directory)) = (path.file_stem(), path.parent()) else {
            return;
        };

        for router in directory.ancestors() {
            let (Some(router_name), Some(parent)) = (router.file_name(), router.parent()) else {
                continue;
            };
            let Ok(directories) = directory.strip_prefix(router) else {
                continue;
            };
            let directories = directories.components().map(|component| component.as_str());

            let route = match router_name {
                "app" => {
                    // Private folders and intercepted routes don't define URLs of their own.
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
                    let route: Box<[Box<str>]> = directories
                        .chain((file_stem != "index").then_some(file_stem))
                        .map(Box::from)
                        .collect();
                    let is_page = match route.as_ref() {
                        [first, ..] if first.as_ref() == "api" => false,
                        [only] => !matches!(only.as_ref(), "_app" | "_document" | "_error"),
                        _ => true,
                    };
                    if !is_page {
                        continue;
                    }
                    route
                }
                _ => continue,
            };

            let project_path = if parent.file_name() == Some("src") {
                parent.parent().unwrap_or(parent)
            } else {
                parent
            };
            pages.push(NextPage {
                project_path: project_path.to_path_buf(),
                route,
            });
        }
    });
    pages
}

/// Like `@next/next/no-html-link-for-pages`, dynamic segments don't match file
/// names such as `report.pdf`, which are usually served from `public`.
fn is_dynamic_value(segment: &str) -> bool {
    !segment.contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(route: &[&str]) -> NextPage {
        NextPage {
            project_path: Utf8PathBuf::from("/"),
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

    #[test]
    fn is_dynamic_value_rejects_file_names() {
        assert!(is_dynamic_value("hello-world"));
        assert!(!is_dynamic_value("report.pdf"));
    }
}
