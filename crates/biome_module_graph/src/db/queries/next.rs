//! Queries over the pages of Next.js applications.

use crate::ModuleDb;
use camino::{Utf8Path, Utf8PathBuf};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct NextPage {
    /// The directory that contains `app`/`pages`, or `src/app`/`src/pages`.
    pub project_path: Utf8PathBuf,
    route: Box<[Box<str>]>,
}

impl NextPage {
    /// `path` is the URL path split into its non-empty segments.
    pub fn matches(&self, path: &[&str]) -> bool {
        route_matches(&self.route, path)
    }
}

/// The default `pageExtensions` of Next.js.
const PAGE_EXTENSIONS: [&str; 4] = ["tsx", "ts", "jsx", "js"];

/// Returns the pages of all projects: rules keep those of their own project.
///
/// Reads only module paths, not their kind, which changes on every edit.
#[salsa::tracked(returns(deref))]
pub fn next_pages(db: &dyn ModuleDb) -> Vec<NextPage> {
    let mut pages = Vec::new();
    db.for_each_module(&mut |module| {
        let path = module.path(db);
        if path
            .extension()
            .is_some_and(|extension| PAGE_EXTENSIONS.contains(&extension))
        {
            pages.extend(pages_of_file(path));
        }
    });
    pages.sort_unstable();
    pages
}

fn pages_of_file(path: &Utf8Path) -> Vec<NextPage> {
    let segments: Vec<&str> = path.iter().collect();
    (0..segments.len())
        .filter_map(|index| {
            let route = route_of_file(segments[index], &segments[index + 1..])?;
            let parent = &segments[..index];
            let project = parent.strip_suffix(&["src"]).unwrap_or(parent);
            Some(NextPage {
                project_path: project.iter().collect(),
                route,
            })
        })
        .collect()
}

/// `router` is `app` or `pages`, and `path` the path of the file inside it.
fn route_of_file(router: &str, path: &[&str]) -> Option<Box<[Box<str>]>> {
    let (file_name, directories) = path.split_last()?;
    let file_stem = Utf8Path::new(*file_name).file_stem()?;
    let route: Vec<&str> = match router {
        "app" => {
            // Private folders and intercepted routes do not define URLs of their own.
            if file_stem != "page"
                || directories
                    .iter()
                    .any(|directory| directory.starts_with('_') || directory.starts_with("(."))
            {
                return None;
            }
            directories
                .iter()
                .copied()
                .filter(|directory| !is_route_group(directory) && !directory.starts_with('@'))
                .collect()
        }
        "pages" => {
            if directories.first() == Some(&"api") || file_stem.starts_with('_') {
                return None;
            }
            directories
                .iter()
                .copied()
                .chain((file_stem != "index").then_some(file_stem))
                .collect()
        }
        _ => return None,
    };
    Some(route.into_iter().map(Box::from).collect())
}

fn is_route_group(directory: &str) -> bool {
    directory.starts_with('(') && directory.ends_with(')')
}

fn route_matches(route: &[Box<str>], path: &[&str]) -> bool {
    let Some((route_segment, route_rest)) = route.split_first() else {
        return path.is_empty();
    };
    if route_segment.starts_with("[[...") {
        return path.iter().all(|segment| is_dynamic_value(segment));
    }
    if route_segment.starts_with("[...") {
        return !path.is_empty() && path.iter().all(|segment| is_dynamic_value(segment));
    }
    let Some((path_segment, path_rest)) = path.split_first() else {
        return false;
    };
    let segment_matches = if route_segment.starts_with('[') {
        is_dynamic_value(path_segment)
    } else {
        route_segment.as_ref() == *path_segment
    };
    segment_matches && route_matches(route_rest, path_rest)
}

/// Like `@next/next/no-html-link-for-pages`, dynamic segments don't match file
/// names such as `report.pdf`, which are usually served from `public`.
fn is_dynamic_value(segment: &str) -> bool {
    !segment.contains('.')
}
