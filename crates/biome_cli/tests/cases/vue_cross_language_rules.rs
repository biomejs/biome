use crate::TestArgs as Args;
use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use camino::Utf8Path;

const BIOME_CONFIG_HTML_FULL_SUPPORT: &str =
    r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#;

const BIOME_CONFIG_FILE_LEVEL_RULE: &str = r#"{
    "html": {
        "experimentalFullSupportEnabled": true,
        "linter": { "enabled": true }
    },
    "linter": {
        "rules": {
            "preset": "none",
            "style": { "useFilenamingConvention": "error" }
        }
    }
}"#;

#[test]
fn file_level_rule_runs_once_per_host_document() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert("biome.json".into(), BIOME_CONFIG_FILE_LEVEL_RULE.as_bytes());
    fs.insert(
        "FooBar.vue".into(),
        r#"<script setup>
const a = 1;
</script>

<template>
  <span>{{ a }}</span>
  <span>{{ a }}</span>
  <span>{{ a }}</span>
</template>
"#
        .as_bytes(),
    );
    fs.insert(
        "BarBaz.astro".into(),
        r#"---
const a = 1;
---
<span>{a}</span>
<span>{a}</span>
<span>{a}</span>
"#
        .as_bytes(),
    );
    fs.insert(
        "Exported.vue".into(),
        r#"<template>
  <span>{{ Exported }}</span>
</template>

<script>
export const Exported = 1;
</script>
"#
        .as_bytes(),
    );
    fs.insert(
        "OnlyExpressions.vue".into(),
        r#"<template>
  <span>{{ first }}</span>
  <span>{{ second }}</span>
  <span>{{ third }}</span>
</template>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--only=style/useFilenamingConvention",
                "FooBar.vue",
                "BarBaz.astro",
                "Exported.vue",
                "OnlyExpressions.vue",
            ]
            .as_slice(),
        ),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "file_level_rule_runs_once_per_host_document",
        fs,
        console,
        result,
    ));
}

#[test]
fn astro_file_level_rule_runs_once_per_host_document() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert("biome.json".into(), BIOME_CONFIG_FILE_LEVEL_RULE.as_bytes());
    fs.insert(
        "AstroComponent.astro".into(),
        r#"<main>
  <h1>Markup only</h1>
</main>
"#
        .as_bytes(),
    );

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "lint",
                "--only=style/useFilenamingConvention",
                "AstroComponent.astro",
            ]
            .as_slice(),
        ),
    );
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "astro_file_level_rule_runs_once_per_host_document",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_undeclared_variables_script_setup_with_functions_and_vars() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"
<template>
	<v-text-field
		:class="copySuccess ? 'text-success' : ''"
		ref="inviteLinkText"
		:value="inviteLink"
		:append-icon="mdiClipboardOutline"
		:messages="copySuccess ? 'Copied' : ''"
		@focus="onFocusHighlightText"
		@click:append="copyInviteLink"
	/>
</template>

<script lang="ts" setup>
import { mdiClipboardOutline } from "@mdi/js";
import { ref, computed } from "vue";
import { useStore } from "@/store";
import { useCopyFromTextbox } from "./composables";

function buildInviteLink(
	currentLocation: string,
	roomName: string,
	shortUrl: string | undefined
): string {
   	if (shortUrl !== undefined) {
  		return `https://${shortUrl}/${roomName}`;
   	}
   	return currentLocation.split("?")[0].toLowerCase();
}

const store = useStore();
const inviteLinkText = ref();

function getInviteLink() {
    return buildInviteLink(window.location.href, store.state.room.name, store.state.shortUrl);
}
const inviteLink = computed(getInviteLink);

function onFocusHighlightText(e) {
    e.target.select();
}

const { copy: copyInviteLink, copySuccess } = useCopyFromTextbox(inviteLink, inviteLinkText);
</script>
"#
        .as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUndeclaredVariables", file.as_str()].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_undeclared_variables_script_setup_with_functions_and_vars",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_undeclared_variables_not_triggered_for_define_props_type_arg() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"
<script setup lang="ts">
defineProps<{ loading?: boolean; disabled?: boolean }>()
</script>
<template>
  <div :class="{ active: loading }" v-if="disabled">test</div>
</template>
"#
        .as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUndeclaredVariables", file.as_str()].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_undeclared_variables_not_triggered_for_define_props_type_arg",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_undeclared_variables_not_triggered_defined_props_options_api() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"
<script>
export default {
  props: {
    loading: Boolean,
    disabled: Boolean,
  },
}
</script>
<template>
  <div :class="{ active: loading }" v-if="disabled">test</div>
</template>
"#
        .as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUndeclaredVariables", file.as_str()].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_undeclared_variables_not_triggered_defined_props_options_api",
        fs,
        console,
        result,
    ));
}

#[test]
fn no_undeclared_variables_not_triggered_for_define_props_type_arg_2() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();
    fs.insert(
        "biome.json".into(),
        BIOME_CONFIG_HTML_FULL_SUPPORT.as_bytes(),
    );
    let file = Utf8Path::new("file.vue");
    fs.insert(
        file.into(),
        r#"
<template>
	<div
		:class="{
			'video-controls': true,
			'in-video': mode === 'in-video',
			'outside-video': mode === 'outside-video',
			'hide': !controlsVisible,
		}"
	>
		<VideoProgressSlider :current-position="sliderPosition" />
		<div class="controls-row2">
			<BasicControls :current-position="truePosition" />
			<TimestampDisplay :current-position="truePosition" />
		</div>
	</div>
</template>

<script lang="ts" setup>
import BasicControls from "./BasicControls.vue";
import TimestampDisplay from "./TimestampDisplay.vue";
import VideoProgressSlider from "./VideoProgressSlider.vue";

withDefaults(
	defineProps<{
		sliderPosition: number;
		truePosition: number;
		controlsVisible: boolean;
		mode: "in-video" | "outside-video";
	}>(),
	{
		controlsVisible: false,
		mode: "in-video",
	}
);
</script>
"#
        .as_bytes(),
    );
    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["lint", "--only=noUndeclaredVariables", file.as_str()].as_slice()),
    );
    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_undeclared_variables_not_triggered_for_define_props_type_arg_2",
        fs,
        console,
        result,
    ));
}
