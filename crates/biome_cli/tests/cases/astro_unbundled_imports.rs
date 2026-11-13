use crate::TestArgs as Args;
use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;

#[test]
fn no_astro_unbundled_imports_explicit_enablement() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        br#"{
            "html": { "experimentalFullSupportEnabled": true },
            "linter": { "rules": { "preset": "none" } }
        }"#,
    );
    fs.insert(
        "file.astro".into(),
        br#"---
import frontmatter from "./frontmatter.js";
---
<script define:vars={{ message: "Hello" }}>
    import { greet } from "./greet.js";
    import "./greet.js";
    greet(message);
</script>
<script>import "./bundled.js";</script>
<script is:inline type="module">import "/module.js";</script>
<script is:inline>import("/dynamic.js");</script>
"#,
    );

    let (fs, disabled) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "file.astro"].as_slice()),
    );
    assert!(disabled.is_ok(), "{disabled:?}\n{console:#?}");
    let (fs, enabled) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noAstroUnbundledImports", "file.astro"].as_slice()),
    );
    assert!(enabled.is_err(), "{enabled:?}\n{console:#?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_astro_unbundled_imports_explicit_enablement",
        fs,
        console,
        disabled.followed_by(enabled),
    ));
}

#[test]
fn astro_proposed_rules_explicit_enablement() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        br#"{
        "html": { "experimentalFullSupportEnabled": true },
        "linter": { "rules": { "preset": "none" } }
    }"#,
    );
    fs.insert(
        "Component.astro".into(),
        br#"---
import Card from "./Card.astro";
import Button from "./Button.svelte";
const posts = ["first"];
const click = () => {};
export function getStaticPaths() { return posts; }
if (!Astro.locals.user) return Astro.redirect("/login");
---
<Card client:visible />
<Button client:load onClick={click} />
"#,
    );
    fs.insert(
        "Styles.astro".into(),
        br#"<style define:vars={{ color: "red" }}>p { color: var(--color); }</style>"#,
    );
    fs.insert(
        "Button.svelte".into(),
        br#"<script>import { Picture } from "astro:assets";</script><Picture />"#,
    );
    fs.insert(
        "src/pages/index.astro".into(),
        b"---\nreturn Astro.redirect('/login');\n---\n",
    );
    let (fs, disabled) = run_cli(fs, &mut console, Args::from(["lint", "."].as_slice()));
    assert!(disabled.is_ok(), "{disabled:?}\n{console:#?}");
    let (fs, enabled) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--only=noAstroInvalidGetStaticPathsReference",
                "--only=noAstroUnserializableClientProps",
                "--only=noAstroRedirectInComponents",
                "--only=noAstroClientDirectiveOnAstroComponent",
                "--only=noAstroComponentInFramework",
                "--only=noAstroUnappliedStyleVars",
                ".",
            ]
            .as_slice(),
        ),
    );
    assert!(enabled.is_err(), "{enabled:?}\n{console:#?}");
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "astro_proposed_rules_explicit_enablement",
        fs,
        console,
        disabled.followed_by(enabled),
    ));
}
