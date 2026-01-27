use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::HtmlFileSource;
use biome_test_utils::has_bogus_nodes_or_empty_slots;

#[ignore]
#[test]
pub fn quick_test() {
    let code = r#"    <script type="text/x-template" id="header-view-template">
      <div class="draggable-header-view"
        @mousedown="startDrag" @touchstart="startDrag"
        @mousemove="onDrag" @touchmove="onDrag"
        @mouseup="stopDrag" @touchend="stopDrag" @mouseleave="stopDrag">
        <svg class="bg" width="320" height="560">
          <path :d="headerPath" fill="3F51B5"></path>
        </svg>
        <div class="header">
          <slot name="header"></slot>
        </div>
        <div class="content" :style="contentPosition">
          <slot name="content"></slot>
        </div>
      </div>
    </script>
"#;

    let source_type = HtmlFileSource::svelte();
    let options = HtmlParseOptions::from(&source_type);
    let root = parse_html(code, options);
    let syntax = root.syntax();
    dbg!(&syntax, root.diagnostics(), root.has_errors());
    if has_bogus_nodes_or_empty_slots(&syntax) {
        panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
    }
}
