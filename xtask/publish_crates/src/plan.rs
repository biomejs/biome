use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, bail};

use crate::{rate_limit::PublishKind, registry::CrateMetadata, workspace::PublishPackage};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlanAction {
    Skip,
    Publish(PublishKind),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanItem {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) checksum: String,
    pub(crate) action: PlanAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PublishPlan {
    pub(crate) items: Vec<PlanItem>,
    pub(crate) fresh: bool,
}

impl PublishPlan {
    pub(crate) fn new(
        packages: &[PublishPackage],
        checksums: &BTreeMap<String, String>,
        remote: &BTreeMap<String, Option<CrateMetadata>>,
    ) -> Result<Self> {
        let mut actions = BTreeMap::new();
        let mut publishing = BTreeSet::new();
        let mut fresh = true;

        for package in packages {
            let checksum = checksums
                .get(&package.name)
                .with_context(|| format!("missing packaged checksum for {}", package.name))?;
            let action = match remote.get(&package.name) {
                Some(None) => PlanAction::Publish(PublishKind::New),
                Some(Some(metadata)) => match metadata.versions.get(&package.version.to_string()) {
                    Some(remote_version) if remote_version.yanked => {
                        bail!(
                            "{} {} is yanked on crates.io",
                            package.name,
                            package.version
                        )
                    }
                    Some(remote_version) if &remote_version.checksum == checksum => {
                        fresh = false;
                        PlanAction::Skip
                    }
                    Some(remote_version) => bail!(
                        "{} {} is already published with checksum {}, expected {}",
                        package.name,
                        package.version,
                        remote_version.checksum,
                        checksum
                    ),
                    None => PlanAction::Publish(PublishKind::Update),
                },
                None => bail!("missing crates.io metadata for {}", package.name),
            };
            if matches!(action, PlanAction::Publish(_)) {
                publishing.insert(package.name.as_str());
            }
            actions.insert(package.name.as_str(), action);
        }

        let items = topological_order(packages, &publishing)?
            .into_iter()
            .map(|package| {
                Ok(PlanItem {
                    name: package.name.clone(),
                    version: package.version.to_string(),
                    checksum: checksums
                        .get(&package.name)
                        .with_context(|| format!("missing packaged checksum for {}", package.name))?
                        .clone(),
                    action: *actions
                        .get(package.name.as_str())
                        .with_context(|| format!("missing plan action for {}", package.name))?,
                })
            })
            .collect::<Result<_>>()?;

        Ok(Self { items, fresh })
    }
}

pub(crate) fn topological_order<'a>(
    packages: &'a [PublishPackage],
    publishing: &BTreeSet<&str>,
) -> Result<Vec<&'a PublishPackage>> {
    let packages_by_name = packages
        .iter()
        .map(|package| (package.name.as_str(), package))
        .collect::<BTreeMap<_, _>>();
    if packages_by_name.len() != packages.len() {
        bail!("duplicate publishable crate name");
    }
    let mut dependents = packages
        .iter()
        .map(|package| (package.name.as_str(), BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    let mut incoming = packages
        .iter()
        .map(|package| (package.name.as_str(), 0usize))
        .collect::<BTreeMap<_, _>>();

    for package in packages {
        for dependency in package
            .dependencies
            .iter()
            .chain(package.dev_dependencies.iter().filter(|dependency| {
                publishing.contains(package.name.as_str())
                    && publishing.contains(dependency.as_str())
            }))
        {
            if !packages_by_name.contains_key(dependency.as_str()) {
                bail!(
                    "{} depends on missing publishable crate {dependency}",
                    package.name
                );
            }
            if dependents
                .get_mut(dependency.as_str())
                .with_context(|| format!("missing dependency node {dependency}"))?
                .insert(package.name.as_str())
            {
                *incoming
                    .get_mut(package.name.as_str())
                    .with_context(|| format!("missing dependent node {}", package.name))? += 1;
            }
        }
    }

    let mut ready = incoming
        .iter()
        .filter_map(|(name, count)| (*count == 0).then_some(*name))
        .collect::<BTreeSet<_>>();
    let mut ordered = Vec::with_capacity(packages.len());

    while let Some(name) = ready.pop_first() {
        ordered.push(
            *packages_by_name
                .get(name)
                .with_context(|| format!("missing publishable crate {name}"))?,
        );
        for dependent in dependents
            .get(name)
            .with_context(|| format!("missing dependents for {name}"))?
        {
            let count = incoming
                .get_mut(dependent)
                .with_context(|| format!("missing incoming edge count for {dependent}"))?;
            *count = count
                .checked_sub(1)
                .with_context(|| format!("invalid incoming edge count for {dependent}"))?;
            if *count == 0 {
                ready.insert(dependent);
            }
        }
    }

    if ordered.len() != packages.len() {
        let cycle = incoming
            .into_iter()
            .filter_map(|(name, count)| (count > 0).then_some(name))
            .collect::<Vec<_>>()
            .join(", ");
        bail!("circular publish dependency detected among: {cycle}");
    }

    Ok(ordered)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checksums(packages: &[PublishPackage]) -> BTreeMap<String, String> {
        packages
            .iter()
            .map(|package| (package.name.clone(), format!("{}-checksum", package.name)))
            .collect()
    }

    #[test]
    fn orders_dependencies_before_dependents_with_lexical_ties() {
        let packages = vec![
            PublishPackage::new("z", &["b", "a"]),
            PublishPackage::new("b", &[]),
            PublishPackage::new("a", &[]),
            PublishPackage::new("c", &["a"]),
        ];

        let order = topological_order(&packages, &BTreeSet::new())
            .unwrap()
            .into_iter()
            .map(|package| package.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(order, ["a", "b", "c", "z"]);
    }

    #[test]
    fn reports_cycles() {
        let packages = vec![
            PublishPackage::new("a", &["b"]),
            PublishPackage::new("b", &["a"]),
        ];

        let error = topological_order(&packages, &BTreeSet::new())
            .unwrap_err()
            .to_string();

        assert_eq!(error, "circular publish dependency detected among: a, b");
    }

    #[test]
    fn development_cycles_only_apply_to_publishing_versions() {
        let mut a = PublishPackage::new("a", &[]);
        a.dev_dependencies.insert("b".to_string());
        let mut b = PublishPackage::new("b", &[]);
        b.dev_dependencies.insert("a".to_string());
        let packages = vec![a, b];

        let existing_order = topological_order(&packages, &BTreeSet::new())
            .unwrap()
            .into_iter()
            .map(|package| package.name.as_str())
            .collect::<Vec<_>>();
        let publishing = BTreeSet::from(["a", "b"]);
        let error = topological_order(&packages, &publishing)
            .unwrap_err()
            .to_string();

        assert_eq!(existing_order, ["a", "b"]);
        assert_eq!(error, "circular publish dependency detected among: a, b");
    }

    #[test]
    fn unpublished_development_dependencies_are_published_first() {
        let mut dependent = PublishPackage::new("a", &[]);
        dependent.dev_dependencies.insert("z".to_string());
        let packages = vec![dependent, PublishPackage::new("z", &[])];

        let order = topological_order(&packages, &BTreeSet::from(["a", "z"]))
            .unwrap()
            .into_iter()
            .map(|package| package.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(order, ["z", "a"]);
    }

    #[test]
    fn skipped_development_dependents_do_not_create_cycles() {
        let mut skipped = PublishPackage::new("a", &[]);
        skipped.dev_dependencies.insert("z".to_string());
        let packages = vec![skipped, PublishPackage::new("z", &["a"])];
        let remote = BTreeMap::from([
            (
                "a".to_string(),
                Some(CrateMetadata {
                    versions: BTreeMap::from([(
                        "1.0.0".to_string(),
                        crate::registry::CrateVersion {
                            checksum: "a-checksum".to_string(),
                            yanked: false,
                        },
                    )]),
                }),
            ),
            ("z".to_string(), None),
        ]);

        let plan = PublishPlan::new(&packages, &checksums(&packages), &remote).unwrap();
        let order = plan
            .items
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(order, ["a", "z"]);
        assert_eq!(plan.items[0].action, PlanAction::Skip);
        assert_eq!(plan.items[1].action, PlanAction::Publish(PublishKind::New));
    }

    #[test]
    fn updated_development_dependencies_are_published_first() {
        let mut dependent = PublishPackage::new("a", &[]);
        dependent.dev_dependencies.insert("z".to_string());
        let packages = vec![dependent, PublishPackage::new("z", &[])];
        let remote = BTreeMap::from([
            (
                "a".to_string(),
                Some(CrateMetadata {
                    versions: BTreeMap::new(),
                }),
            ),
            (
                "z".to_string(),
                Some(CrateMetadata {
                    versions: BTreeMap::new(),
                }),
            ),
        ]);

        let plan = PublishPlan::new(&packages, &checksums(&packages), &remote).unwrap();
        let order = plan
            .items
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(order, ["z", "a"]);
    }

    #[test]
    fn skips_only_matching_remote_versions() {
        let packages = vec![PublishPackage::new("a", &[])];
        let remote = BTreeMap::from([(
            "a".to_string(),
            Some(CrateMetadata {
                versions: BTreeMap::from([(
                    "1.0.0".to_string(),
                    crate::registry::CrateVersion {
                        checksum: "a-checksum".to_string(),
                        yanked: false,
                    },
                )]),
            }),
        )]);

        let plan = PublishPlan::new(&packages, &checksums(&packages), &remote).unwrap();

        assert_eq!(plan.items[0].action, PlanAction::Skip);
        assert!(!plan.fresh);
    }

    #[test]
    fn rejects_yanked_remote_versions() {
        let packages = vec![PublishPackage::new("a", &[])];
        let remote = BTreeMap::from([(
            "a".to_string(),
            Some(CrateMetadata {
                versions: BTreeMap::from([(
                    "1.0.0".to_string(),
                    crate::registry::CrateVersion {
                        checksum: "a-checksum".to_string(),
                        yanked: true,
                    },
                )]),
            }),
        )]);

        let error = PublishPlan::new(&packages, &checksums(&packages), &remote)
            .unwrap_err()
            .to_string();

        assert_eq!(error, "a 1.0.0 is yanked on crates.io");
    }

    #[test]
    fn rejects_mismatched_remote_versions() {
        let packages = vec![PublishPackage::new("a", &[])];
        let remote = BTreeMap::from([(
            "a".to_string(),
            Some(CrateMetadata {
                versions: BTreeMap::from([(
                    "1.0.0".to_string(),
                    crate::registry::CrateVersion {
                        checksum: "other".to_string(),
                        yanked: false,
                    },
                )]),
            }),
        )]);

        let error = PublishPlan::new(&packages, &checksums(&packages), &remote)
            .unwrap_err()
            .to_string();

        assert!(error.contains("already published with checksum other"));
    }
}
