use std::{collections::BTreeMap, env, path::Path};

use anyhow::{Context, Result, bail};

use crate::{
    cargo::{CargoRunner, SystemCargo},
    plan::{PlanAction, PublishPlan},
    rate_limit::{PublishKind, PublishRateLimiter},
    registry::{CrateMetadata, CratesIoRegistry, Registry},
    workspace::{PublishPackage, Workspace},
};

pub(crate) fn verify(directory: &Path, allow_dirty: bool) -> Result<()> {
    let workspace = Workspace::from_directory(directory, allow_dirty)?;
    println!(
        "Verifying {} public crates at version {}",
        workspace.packages.len(),
        workspace.version
    );
    SystemCargo::new().verify(&workspace, allow_dirty)?;
    if !allow_dirty {
        workspace.ensure_unchanged()?;
        println!(
            "Dry-run verification succeeded for Git tree {}",
            workspace.current_tree()?
        );
    }
    Ok(())
}

pub(crate) fn publish(
    directory: &Path,
    execute: bool,
    verified_revision: Option<&str>,
) -> Result<()> {
    if !execute && verified_revision.is_some() {
        bail!("--verified-revision requires --execute");
    }
    let workspace = Workspace::from_directory(directory, false)?;
    if execute {
        let verified_revision =
            verified_revision.context("--verified-revision is required together with --execute")?;
        workspace.ensure_verified_revision(verified_revision)?;
    }
    let mut cargo = SystemCargo::new();
    let checksums = cargo.package(&workspace)?;
    workspace.ensure_unchanged()?;
    let mut registry = CratesIoRegistry::new()?;
    let remote = query_remote(&workspace.packages, &mut registry)?;
    let plan = PublishPlan::new(&workspace.packages, &checksums, &remote)?;
    workspace.ensure_unchanged()?;
    print_plan(&plan);
    if !execute {
        println!("No crates were published. Pass --execute to apply this plan.");
        return Ok(());
    }

    let use_initial_burst = plan.fresh && is_serialized_publish_workflow();
    let limiter = PublishRateLimiter::new(use_initial_burst)?;
    execute_plan(
        &workspace,
        &plan,
        &mut cargo,
        &mut registry,
        &limiter,
        || workspace.ensure_unchanged(),
    )?;
    workspace.ensure_unchanged()
}

fn is_serialized_publish_workflow() -> bool {
    let actions = env::var("GITHUB_ACTIONS").ok();
    let workflow = env::var("GITHUB_WORKFLOW").ok();
    let repository = env::var("GITHUB_REPOSITORY").ok();
    let run_attempt = env::var("GITHUB_RUN_ATTEMPT").ok();
    is_initial_publish_workflow(
        actions.as_deref(),
        workflow.as_deref(),
        repository.as_deref(),
        run_attempt.as_deref(),
    )
}

fn is_initial_publish_workflow(
    actions: Option<&str>,
    workflow: Option<&str>,
    repository: Option<&str>,
    run_attempt: Option<&str>,
) -> bool {
    actions == Some("true")
        && workflow == Some("Publish crates")
        && repository == Some("biomejs/biome")
        && run_attempt == Some("1")
}

fn query_remote<R: Registry>(
    packages: &[PublishPackage],
    registry: &mut R,
) -> Result<BTreeMap<String, Option<CrateMetadata>>> {
    packages
        .iter()
        .map(|package| {
            println!("Checking crates.io metadata for {}", package.name);
            registry
                .crate_metadata(&package.name)
                .with_context(|| format!("failed to query crates.io for {}", package.name))
                .map(|metadata| (package.name.clone(), metadata))
        })
        .collect()
}

fn print_plan(plan: &PublishPlan) {
    println!("Crate publication plan:");
    for (index, item) in plan.items.iter().enumerate() {
        let action = match item.action {
            PlanAction::Skip => "skip",
            PlanAction::Publish(PublishKind::New) => "publish new crate",
            PlanAction::Publish(PublishKind::Update) => "publish update",
        };
        println!(
            "  {:>2}. {} {}: {action}",
            index + 1,
            item.name,
            item.version
        );
    }
}

trait RateGate {
    fn wait(&self, kind: PublishKind) -> Result<()>;
}

impl RateGate for PublishRateLimiter {
    fn wait(&self, kind: PublishKind) -> Result<()> {
        PublishRateLimiter::wait(self, kind)
    }
}

fn execute_plan<R, C, G, F>(
    workspace: &Workspace,
    plan: &PublishPlan,
    cargo: &mut C,
    registry: &mut R,
    gate: &G,
    mut ensure_unchanged: F,
) -> Result<()>
where
    R: Registry,
    C: CargoRunner,
    G: RateGate,
    F: FnMut() -> Result<()>,
{
    let packages = workspace
        .packages
        .iter()
        .map(|package| (package.name.as_str(), package))
        .collect::<BTreeMap<_, _>>();
    let mut published = 0usize;
    let mut skipped = 0usize;

    for (index, item) in plan.items.iter().enumerate() {
        match item.action {
            PlanAction::Skip => {
                registry.wait_until_indexed(&item.name, &item.version, &item.checksum)?;
                skipped += 1;
                println!(
                    "[{}/{}] {} {} already published with matching checksum",
                    index + 1,
                    plan.items.len(),
                    item.name,
                    item.version
                );
            }
            PlanAction::Publish(kind) => {
                ensure_unchanged()?;
                gate.wait(kind)?;
                ensure_unchanged()?;
                println!(
                    "[{}/{}] Publishing {} {}",
                    index + 1,
                    plan.items.len(),
                    item.name,
                    item.version
                );
                let package = packages
                    .get(item.name.as_str())
                    .with_context(|| format!("missing publish package {}", item.name))?;
                let success = cargo.publish(workspace, package)?;
                if !success {
                    let metadata = registry.crate_metadata(&item.name)?;
                    let uploaded = metadata
                        .as_ref()
                        .and_then(|metadata| metadata.versions.get(&item.version))
                        .is_some_and(|version| {
                            !version.yanked && version.checksum == item.checksum
                        });
                    if !uploaded {
                        bail!(
                            "cargo failed to publish {} {}, and crates.io does not contain the expected checksum",
                            item.name,
                            item.version
                        );
                    }
                }
                registry.wait_until_published(&item.name, &item.version, &item.checksum)?;
                published += 1;
            }
        }
    }

    println!(
        "Crate publication complete: {published} published, {skipped} skipped, {} total",
        plan.items.len()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    use super::*;

    struct FakeCargo {
        results: VecDeque<bool>,
        published: Vec<String>,
    }

    impl CargoRunner for FakeCargo {
        fn verify(&mut self, _workspace: &Workspace, _allow_dirty: bool) -> Result<()> {
            Ok(())
        }

        fn package(&mut self, _workspace: &Workspace) -> Result<BTreeMap<String, String>> {
            Ok(BTreeMap::new())
        }

        fn publish(&mut self, _workspace: &Workspace, package: &PublishPackage) -> Result<bool> {
            self.published.push(package.name.clone());
            Ok(self.results.pop_front().unwrap_or(true))
        }
    }

    struct FakeRegistry {
        metadata: BTreeMap<String, VecDeque<Option<CrateMetadata>>>,
        waited: Vec<String>,
    }

    impl Registry for FakeRegistry {
        fn crate_metadata(&mut self, name: &str) -> Result<Option<CrateMetadata>> {
            let responses = self.metadata.get_mut(name).unwrap();
            if responses.len() > 1 {
                Ok(responses.pop_front().unwrap())
            } else {
                Ok(responses.front().unwrap().clone())
            }
        }

        fn wait_until_indexed(
            &mut self,
            name: &str,
            _version: &str,
            _checksum: &str,
        ) -> Result<()> {
            self.waited.push(name.to_string());
            Ok(())
        }

        fn wait_until_published(
            &mut self,
            name: &str,
            _version: &str,
            _checksum: &str,
        ) -> Result<()> {
            self.waited.push(name.to_string());
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeGate {
        kinds: RefCell<Vec<PublishKind>>,
    }

    impl RateGate for FakeGate {
        fn wait(&self, kind: PublishKind) -> Result<()> {
            self.kinds.borrow_mut().push(kind);
            Ok(())
        }
    }

    struct RecordingCargo {
        events: Rc<RefCell<Vec<String>>>,
    }

    impl CargoRunner for RecordingCargo {
        fn verify(&mut self, _workspace: &Workspace, _allow_dirty: bool) -> Result<()> {
            Ok(())
        }

        fn package(&mut self, _workspace: &Workspace) -> Result<BTreeMap<String, String>> {
            Ok(BTreeMap::new())
        }

        fn publish(&mut self, _workspace: &Workspace, package: &PublishPackage) -> Result<bool> {
            self.events
                .borrow_mut()
                .push(format!("publish {}", package.name));
            Ok(true)
        }
    }

    struct RecordingRegistry {
        events: Rc<RefCell<Vec<String>>>,
    }

    impl Registry for RecordingRegistry {
        fn crate_metadata(&mut self, _name: &str) -> Result<Option<CrateMetadata>> {
            Ok(None)
        }

        fn wait_until_indexed(
            &mut self,
            name: &str,
            _version: &str,
            _checksum: &str,
        ) -> Result<()> {
            self.events.borrow_mut().push(format!("indexed {name}"));
            Ok(())
        }

        fn wait_until_published(
            &mut self,
            name: &str,
            _version: &str,
            _checksum: &str,
        ) -> Result<()> {
            self.events.borrow_mut().push(format!("ready {name}"));
            Ok(())
        }
    }

    fn publish_item(name: &str) -> crate::plan::PlanItem {
        crate::plan::PlanItem {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            checksum: format!("{name}-checksum"),
            action: PlanAction::Publish(PublishKind::Update),
        }
    }

    #[test]
    fn only_the_first_checked_in_workflow_attempt_can_use_a_burst() {
        assert!(is_initial_publish_workflow(
            Some("true"),
            Some("Publish crates"),
            Some("biomejs/biome"),
            Some("1")
        ));
        assert!(!is_initial_publish_workflow(
            Some("true"),
            Some("Publish crates"),
            Some("biomejs/biome"),
            Some("2")
        ));
    }

    #[test]
    fn reconciles_an_ambiguous_cargo_failure() {
        let workspace = Workspace::new(vec![PublishPackage::new("a", &[])]);
        let plan = PublishPlan {
            items: vec![publish_item("a")],
            fresh: true,
        };
        let mut cargo = FakeCargo {
            results: VecDeque::from([false]),
            published: Vec::new(),
        };
        let mut registry = FakeRegistry {
            metadata: BTreeMap::from([(
                "a".to_string(),
                VecDeque::from([Some(CrateMetadata {
                    versions: BTreeMap::from([(
                        "1.0.0".to_string(),
                        crate::registry::CrateVersion {
                            checksum: "a-checksum".to_string(),
                            yanked: false,
                        },
                    )]),
                })]),
            )]),
            waited: Vec::new(),
        };
        let gate = FakeGate::default();

        execute_plan(&workspace, &plan, &mut cargo, &mut registry, &gate, || {
            Ok(())
        })
        .unwrap();

        assert_eq!(cargo.published, ["a"]);
        assert_eq!(registry.waited, ["a"]);
        assert_eq!(*gate.kinds.borrow(), [PublishKind::Update]);
    }

    #[test]
    fn stops_after_a_definite_cargo_failure() {
        let workspace = Workspace::new(vec![
            PublishPackage::new("a", &[]),
            PublishPackage::new("b", &["a"]),
        ]);
        let plan = PublishPlan {
            items: vec![publish_item("a"), publish_item("b")],
            fresh: true,
        };
        let mut cargo = FakeCargo {
            results: VecDeque::from([false]),
            published: Vec::new(),
        };
        let mut registry = FakeRegistry {
            metadata: BTreeMap::from([("a".to_string(), VecDeque::from([None]))]),
            waited: Vec::new(),
        };
        let gate = FakeGate::default();

        let error = execute_plan(&workspace, &plan, &mut cargo, &mut registry, &gate, || {
            Ok(())
        })
        .unwrap_err()
        .to_string();

        assert!(error.contains("crates.io does not contain the expected checksum"));
        assert_eq!(cargo.published, ["a"]);
        assert!(registry.waited.is_empty());
    }

    #[test]
    fn waits_for_registry_readiness_before_publishing_a_dependent() {
        let workspace = Workspace::new(vec![
            PublishPackage::new("a", &[]),
            PublishPackage::new("b", &["a"]),
        ]);
        let plan = PublishPlan {
            items: vec![publish_item("a"), publish_item("b")],
            fresh: true,
        };
        let events = Rc::new(RefCell::new(Vec::new()));
        let mut cargo = RecordingCargo {
            events: Rc::clone(&events),
        };
        let mut registry = RecordingRegistry {
            events: Rc::clone(&events),
        };

        execute_plan(
            &workspace,
            &plan,
            &mut cargo,
            &mut registry,
            &FakeGate::default(),
            || Ok(()),
        )
        .unwrap();

        assert_eq!(
            *events.borrow(),
            ["publish a", "ready a", "publish b", "ready b"]
        );
    }

    #[test]
    fn waits_for_a_skipped_dependency_to_reach_the_index() {
        let workspace = Workspace::new(vec![
            PublishPackage::new("a", &[]),
            PublishPackage::new("b", &["a"]),
        ]);
        let plan = PublishPlan {
            items: vec![
                crate::plan::PlanItem {
                    action: PlanAction::Skip,
                    ..publish_item("a")
                },
                publish_item("b"),
            ],
            fresh: false,
        };
        let events = Rc::new(RefCell::new(Vec::new()));
        let mut cargo = RecordingCargo {
            events: Rc::clone(&events),
        };
        let mut registry = RecordingRegistry {
            events: Rc::clone(&events),
        };

        execute_plan(
            &workspace,
            &plan,
            &mut cargo,
            &mut registry,
            &FakeGate::default(),
            || Ok(()),
        )
        .unwrap();

        assert_eq!(*events.borrow(), ["indexed a", "publish b", "ready b"]);
    }

    #[test]
    fn fully_published_releases_are_noops() {
        let workspace = Workspace::new(vec![PublishPackage::new("a", &[])]);
        let plan = PublishPlan {
            items: vec![crate::plan::PlanItem {
                action: PlanAction::Skip,
                ..publish_item("a")
            }],
            fresh: false,
        };
        let mut cargo = FakeCargo {
            results: VecDeque::new(),
            published: Vec::new(),
        };
        let mut registry = FakeRegistry {
            metadata: BTreeMap::new(),
            waited: Vec::new(),
        };
        let gate = FakeGate::default();

        execute_plan(&workspace, &plan, &mut cargo, &mut registry, &gate, || {
            Ok(())
        })
        .unwrap();

        assert!(cargo.published.is_empty());
        assert_eq!(registry.waited, ["a"]);
        assert!(gate.kinds.borrow().is_empty());
    }
}
