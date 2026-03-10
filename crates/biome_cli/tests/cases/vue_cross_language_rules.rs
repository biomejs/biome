use crate::run_cli;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::MemoryFileSystem;
use bpaf::Args;
use camino::Utf8Path;

const BIOME_CONFIG_HTML_FULL_SUPPORT: &str =
    r#"{ "html": { "linter": {"enabled": true}, "experimentalFullSupportEnabled": true } }"#;

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
