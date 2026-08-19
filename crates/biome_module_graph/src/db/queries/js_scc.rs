use crate::{JsImportPath, ModuleDb, ModuleGraphGeneration, ModuleInfoKind};
use biome_fs::is_node_modules_path;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashMap;

/// Strongly connected components of the JavaScript import graph.
///
/// Two modules belong to the same component when each is reachable from the
/// other by following imports. Resolved edges between indexed JavaScript
/// modules outside `node_modules` are included, so callers may use this as a
/// conservative prefilter when they traverse a narrower graph.
///
/// See: <https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm>
#[derive(Debug, Eq, PartialEq)]
pub struct JsModuleSccs {
    component_by_path: FxHashMap<Utf8PathBuf, u32>,
    component_sizes: Box<[u32]>,
}

impl JsModuleSccs {
    /// Returns whether `from` and `to` belong to the same component containing
    /// more than one module.
    pub fn contains_cycle_between(&self, from: &Utf8Path, to: &Utf8Path) -> bool {
        // are they in the same component?
        let (Some(&from_component), Some(&to_component)) = (
            self.component_by_path.get(from),
            self.component_by_path.get(to),
        ) else {
            // if not, then there can't possibly be a cycle between the 2 paths.
            return false;
        };

        from_component == to_component && self.component_sizes[from_component as usize] > 1
    }
}

/// Returns the strongly connected components of the JavaScript import graph.
#[salsa::tracked(no_eq, returns(ref))]
pub fn js_module_sccs(db: &dyn ModuleDb, generation: ModuleGraphGeneration) -> JsModuleSccs {
    let _ = generation.value(db);

    let mut id_by_path = FxHashMap::default();

    db.for_each_module(&mut |module| {
        let path = module.path(db);
        if matches!(module.kind(db), ModuleInfoKind::Js(_)) && !is_node_modules_path(path) {
            id_by_path.insert(path.to_path_buf(), id_by_path.len() as u32);
        }
    });

    let mut edges = vec![Vec::new(); id_by_path.len()];
    db.for_each_module(&mut |module| {
        let ModuleInfoKind::Js(module_info) = module.kind(db) else {
            return;
        };
        let path = module.path(db);
        let Some(&from_id) = id_by_path.get(path) else {
            return;
        };

        for JsImportPath { resolved_path, .. } in module_info.all_import_paths() {
            if let Some(target) = resolved_path.as_path()
                && target != path
                && let Some(&to_id) = id_by_path.get(target)
            {
                edges[from_id as usize].push(to_id);
            }
        }
    });

    let (component_by_id, component_sizes) = compute_sccs(&edges);
    let component_by_path = id_by_path
        .into_iter()
        .map(|(path, id)| (path, component_by_id[id as usize]))
        .collect();

    JsModuleSccs {
        component_by_path,
        component_sizes: component_sizes.into_boxed_slice(),
    }
}

/// Computes strongly connected components with Kosaraju's algorithm
///
/// See: <https://en.wikipedia.org/wiki/Kosaraju%27s_algorithm>
pub(super) fn compute_sccs(edges: &[Vec<u32>]) -> (Vec<u32>, Vec<u32>) {
    let node_count = edges.len();
    let mut visited = vec![false; node_count];
    let mut next_edge = vec![0u32; node_count];
    let mut finish_order = Vec::with_capacity(node_count);
    let mut stack = Vec::new();

    // First pass: DFS over the forward edges, recording each node in
    // post-order once all its successors are explored. `next_edge` tracks a
    // node's progress through its edge list so it can pause on the stack while
    // a successor is explored.
    for start in 0..node_count as u32 {
        if visited[start as usize] {
            continue;
        }
        visited[start as usize] = true;
        stack.push(start);

        while let Some(&node) = stack.last() {
            let node = node as usize;
            if let Some(&next) = edges[node].get(next_edge[node] as usize) {
                next_edge[node] += 1;
                if !visited[next as usize] {
                    visited[next as usize] = true;
                    stack.push(next);
                }
            } else {
                finish_order.push(node as u32);
                stack.pop();
            }
        }
    }

    // Second pass: DFS over the transposed graph, starting nodes in reverse
    // post-order. Each traversal that starts on an unassigned node reaches
    // exactly the members of one component.
    let mut reverse_edges = vec![Vec::new(); node_count];
    for (from, targets) in edges.iter().enumerate() {
        for &to in targets {
            reverse_edges[to as usize].push(from as u32);
        }
    }

    let mut component_by_id = vec![u32::MAX; node_count];
    let mut component_sizes = Vec::new();

    for &start in finish_order.iter().rev() {
        if component_by_id[start as usize] != u32::MAX {
            continue;
        }

        let component = component_sizes.len() as u32;
        component_sizes.push(0);
        stack.push(start);

        while let Some(node) = stack.pop() {
            let node = node as usize;
            if component_by_id[node] != u32::MAX {
                continue;
            }

            component_by_id[node] = component;
            component_sizes[component as usize] += 1;
            stack.extend(
                reverse_edges[node]
                    .iter()
                    .copied()
                    .filter(|predecessor| component_by_id[*predecessor as usize] == u32::MAX),
            );
        }
    }

    (component_by_id, component_sizes)
}

#[cfg(test)]
mod tests {
    use super::{JsModuleSccs, compute_sccs};
    use camino::{Utf8Path, Utf8PathBuf};

    fn module_sccs(paths: &[&str], edges: &[Vec<u32>]) -> JsModuleSccs {
        assert_eq!(paths.len(), edges.len());
        let (component_by_id, component_sizes) = compute_sccs(edges);
        let component_by_path = paths
            .iter()
            .enumerate()
            .map(|(id, path)| (Utf8PathBuf::from(*path), component_by_id[id]))
            .collect();

        JsModuleSccs {
            component_by_path,
            component_sizes: component_sizes.into_boxed_slice(),
        }
    }

    #[test]
    fn separates_acyclic_nodes() {
        let (components, sizes) = compute_sccs(&[vec![1], vec![2], vec![]]);

        assert_ne!(components[0], components[1]);
        assert_ne!(components[1], components[2]);
        assert_ne!(components[0], components[2]);
        assert!(components.iter().all(|&id| sizes[id as usize] == 1));
    }

    #[test]
    fn groups_mutually_reachable_nodes() {
        let (components, sizes) =
            compute_sccs(&[vec![1], vec![2], vec![0], vec![0], vec![5], vec![4]]);

        assert_eq!(components[0], components[1]);
        assert_eq!(components[1], components[2]);
        assert_eq!(sizes[components[0] as usize], 3);
        assert_ne!(components[3], components[0]);
        assert_eq!(components[4], components[5]);
        assert_eq!(sizes[components[4] as usize], 2);
    }

    #[test]
    fn importing_into_cycle_does_not_join_cycle() {
        let (components, sizes) = compute_sccs(&[vec![1], vec![0], vec![0]]);

        assert_eq!(components[0], components[1]);
        assert_eq!(sizes[components[0] as usize], 2);
        assert_ne!(components[2], components[0]);
        assert_eq!(sizes[components[2] as usize], 1);
    }

    #[test]
    fn convergence_without_cycle_separates_all_nodes() {
        let (components, sizes) = compute_sccs(&[vec![1, 2], vec![3], vec![3], vec![]]);

        for (node, &component) in components.iter().enumerate() {
            assert_eq!(sizes[component as usize], 1);
            assert!(components[..node].iter().all(|&other| other != component));
        }
    }

    #[test]
    fn chord_does_not_split_cycle() {
        let (components, sizes) = compute_sccs(&[vec![1], vec![2, 3], vec![3], vec![0]]);

        assert!(
            components
                .iter()
                .all(|&component| component == components[0])
        );
        assert_eq!(sizes[components[0] as usize], 4);
    }

    #[test]
    fn contains_cycle_between_nodes_in_cycle() {
        let sccs = module_sccs(&["/a.js", "/b.js"], &[vec![1], vec![0]]);

        assert!(sccs.contains_cycle_between(Utf8Path::new("/a.js"), Utf8Path::new("/b.js")));
        assert!(sccs.contains_cycle_between(Utf8Path::new("/b.js"), Utf8Path::new("/a.js")));
    }

    #[test]
    fn does_not_contain_cycle_for_edge_exiting_cycle() {
        let sccs = module_sccs(&["/a.js", "/b.js", "/c.js"], &[vec![1, 2], vec![0], vec![]]);

        assert!(sccs.contains_cycle_between(Utf8Path::new("/a.js"), Utf8Path::new("/b.js")));
        assert!(!sccs.contains_cycle_between(Utf8Path::new("/a.js"), Utf8Path::new("/c.js")));
    }

    #[test]
    fn does_not_contain_cycle_for_single_self_import() {
        let sccs = module_sccs(&["/a.js"], &[vec![0]]);

        assert!(!sccs.contains_cycle_between(Utf8Path::new("/a.js"), Utf8Path::new("/a.js")));
    }

    #[test]
    fn does_not_contain_cycle_between_unrelated_cycles() {
        let sccs = module_sccs(
            &["/a.js", "/b.js", "/c.js", "/d.js"],
            &[vec![1], vec![0], vec![3], vec![2]],
        );

        assert!(!sccs.contains_cycle_between(Utf8Path::new("/a.js"), Utf8Path::new("/c.js")));
    }

    #[test]
    fn does_not_contain_cycle_for_unknown_path() {
        let sccs = module_sccs(&["/a.js", "/b.js"], &[vec![1], vec![0]]);

        assert!(!sccs.contains_cycle_between(Utf8Path::new("/a.js"), Utf8Path::new("/unknown.js")));
    }
}
