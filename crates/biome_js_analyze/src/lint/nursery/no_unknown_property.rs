use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{AnyJsxElementName,  JsxAttribute, jsx_ext::AnyJsxElement};
use biome_rowan::AstNode;
use biome_rowan::{AstNodeList, TextRange};
use biome_rule_options::no_unknown_property::NoUnknownPropertyOptions;
use regex::Regex;

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub NoUnknownProperty {
        version: "next",
        name: "noUnknownProperty",
        language: "js",
        recommended: false,
    }
}
const ATTRIBUTE_TAGS_MAP: &[(&str, &[&str])] = &[
    ("abbr", &["th", "td"]),
    (
        "align",
        &[
            "applet", "caption", "col", "colgroup", "hr", "iframe", "img", "table", "tbody", "td",
            "tfoot", "th", "thead", "tr",
        ],
    ),
    ("allowFullScreen", &["iframe", "video"]),
    ("as", &["link"]),
    ("autoPictureInPicture", &["video"]),
    ("charset", &["meta"]),
    ("checked", &["input"]),
    ("controls", &["audio", "video"]),
    ("controlsList", &["audio", "video"]),
    (
        "crossOrigin",
        &["script", "img", "video", "audio", "link", "image"],
    ),
    ("disablePictureInPicture", &["video"]),
    ("disableRemotePlayback", &["audio", "video"]),
    ("displayStyle", &["math"]),
    ("download", &["a", "area"]),
    (
        "fill",
        &[
            "altGlyph",
            "circle",
            "ellipse",
            "g",
            "line",
            "marker",
            "mask",
            "path",
            "polygon",
            "polyline",
            "rect",
            "svg",
            "symbol",
            "text",
            "textPath",
            "tref",
            "tspan",
            "use",
            "animate",
            "animateColor",
            "animateMotion",
            "animateTransform",
            "set",
        ],
    ),
    ("focusable", &["svg"]),
    ("imageSizes", &["link"]),
    ("imageSrcSet", &["link"]),
    ("loop", &["audio", "video"]),
    ("mozAllowFullScreen", &["iframe", "video"]),
    ("muted", &["audio", "video"]),
    ("noModule", &["script"]),
    ("onAbort", &["audio", "video"]),
    ("onCanPlay", &["audio", "video"]),
    ("onCanPlayThrough", &["audio", "video"]),
    ("onCancel", &["dialog"]),
    ("onClose", &["dialog"]),
    ("onDurationChange", &["audio", "video"]),
    ("onEmptied", &["audio", "video"]),
    ("onEncrypted", &["audio", "video"]),
    ("onEnded", &["audio", "video"]),
    (
        "onError",
        &[
            "audio", "video", "img", "link", "source", "script", "picture", "iframe",
        ],
    ),
    (
        "onLoad",
        &[
            "script", "img", "link", "picture", "iframe", "object", "source",
        ],
    ),
    ("onLoadStart", &["audio", "video"]),
    ("onLoadedData", &["audio", "video"]),
    ("onLoadedMetadata", &["audio", "video"]),
    ("onPause", &["audio", "video"]),
    ("onPlay", &["audio", "video"]),
    ("onPlaying", &["audio", "video"]),
    ("onProgress", &["audio", "video"]),
    ("onRateChange", &["audio", "video"]),
    ("onResize", &["audio", "video"]),
    ("onSeeked", &["audio", "video"]),
    ("onSeeking", &["audio", "video"]),
    ("onStalled", &["audio", "video"]),
    ("onSuspend", &["audio", "video"]),
    ("onTimeUpdate", &["audio", "video"]),
    ("onVolumeChange", &["audio", "video"]),
    ("onWaiting", &["audio", "video"]),
    ("playsInline", &["video"]),
    ("poster", &["video"]),
    ("preload", &["audio", "video"]),
    ("property", &["meta"]),
    ("returnValue", &["dialog"]),
    ("scrolling", &["iframe"]),
    (
        "valign",
        &[
            "tr", "td", "th", "thead", "tbody", "tfoot", "colgroup", "col",
        ],
    ),
    ("viewBox", &["marker", "pattern", "svg", "symbol", "view"]),
    ("webkitAllowFullScreen", &["iframe", "video"]),
    ("webkitDirectory", &["input"]),
];

pub fn get_allowed_tags(attribute: &str) -> Option<&'static [&'static str]> {
    ATTRIBUTE_TAGS_MAP
        .binary_search_by_key(&attribute, |&(key, _)| key)
        .ok()
        .map(|idx| ATTRIBUTE_TAGS_MAP[idx].1)
}

const ARIA_PROPERTIES: [&str; 53] = [
    // See https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes
    // Global attributes
    "aria-atomic",
    "aria-braillelabel",
    "aria-brailleroledescription",
    "aria-busy",
    "aria-controls",
    "aria-current",
    "aria-describedby",
    "aria-description",
    "aria-details",
    "aria-disabled",
    "aria-dropeffect",
    "aria-errormessage",
    "aria-flowto",
    "aria-grabbed",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-live",
    "aria-owns",
    "aria-relevant",
    "aria-roledescription",
    // Widget attributes
    "aria-autocomplete",
    "aria-checked",
    "aria-expanded",
    "aria-level",
    "aria-modal",
    "aria-multiline",
    "aria-multiselectable",
    "aria-orientation",
    "aria-placeholder",
    "aria-pressed",
    "aria-readonly",
    "aria-required",
    "aria-selected",
    "aria-sort",
    "aria-valuemax",
    "aria-valuemin",
    "aria-valuenow",
    "aria-valuetext",
    // Relationship attributes
    "aria-activedescendant",
    "aria-colcount",
    "aria-colindex",
    "aria-colindextext",
    "aria-colspan",
    "aria-posinset",
    "aria-rowcount",
    "aria-rowindex",
    "aria-rowindextext",
    "aria-rowspan",
    "aria-setsize",
];

const DOM_PROPERTIES_IGNORE_CASE: [&str; 5] = [
    "allowFullScreen",
    "charset",
    "mozAllowFullScreen",
    "webkitAllowFullScreen",
    "webkitDirectory",
];

const DOM_ATTRIBUTE_NAMES: &[(&str, &str)] = &[
    ("class", "className"),
    ("for", "htmlFor"),
    ("maxlength", "maxLength"),
    ("readonly", "readOnly"),
    ("tabindex", "tabIndex"),
];

const SVGDOM_ATTRIBUTE_NAMES: &[(&str, &str)] = &[
    ("accent-height", "accentHeight"),
    ("alignment-baseline", "alignmentBaseline"),
    ("arabic-form", "arabicForm"),
    ("baseline-shift", "baselineShift"),
    ("cap-height", "capHeight"),
    ("clip-path", "clipPath"),
    ("clip-rule", "clipRule"),
    ("color-interpolation", "colorInterpolation"),
    ("color-interpolation-filters", "colorInterpolationFilters"),
    ("color-profile", "colorProfile"),
    ("color-rendering", "colorRendering"),
    ("dominant-baseline", "dominantBaseline"),
    ("enable-background", "enableBackground"),
    ("fill-opacity", "fillOpacity"),
    ("fill-rule", "fillRule"),
    ("flood-color", "floodColor"),
    ("flood-opacity", "floodOpacity"),
    ("font-family", "fontFamily"),
    ("font-size", "fontSize"),
    ("font-size-adjust", "fontSizeAdjust"),
    ("font-stretch", "fontStretch"),
    ("font-style", "fontStyle"),
    ("font-variant", "fontVariant"),
    ("font-weight", "fontWeight"),
    ("glyph-name", "glyphName"),
    ("glyph-orientation-horizontal", "glyphOrientationHorizontal"),
    ("glyph-orientation-vertical", "glyphOrientationVertical"),
    ("horiz-adv-x", "horizAdvX"),
    ("horiz-origin-x", "horizOriginX"),
    ("image-rendering", "imageRendering"),
    ("letter-spacing", "letterSpacing"),
    ("lighting-color", "lightingColor"),
    ("marker-end", "markerEnd"),
    ("marker-mid", "markerMid"),
    ("marker-start", "markerStart"),
    ("overline-position", "overlinePosition"),
    ("overline-thickness", "overlineThickness"),
    ("paint-order", "paintOrder"),
    ("panose-1", "panose1"),
    ("pointer-events", "pointerEvents"),
    ("rendering-intent", "renderingIntent"),
    ("shape-rendering", "shapeRendering"),
    ("stop-color", "stopColor"),
    ("stop-opacity", "stopOpacity"),
    ("strikethrough-position", "strikethroughPosition"),
    ("strikethrough-thickness", "strikethroughThickness"),
    ("stroke-dasharray", "strokeDasharray"),
    ("stroke-dashoffset", "strokeDashoffset"),
    ("stroke-linecap", "strokeLinecap"),
    ("stroke-linejoin", "strokeLinejoin"),
    ("stroke-miterlimit", "strokeMiterlimit"),
    ("stroke-opacity", "strokeOpacity"),
    ("stroke-width", "strokeWidth"),
    ("text-anchor", "textAnchor"),
    ("text-decoration", "textDecoration"),
    ("text-rendering", "textRendering"),
    ("underline-position", "underlinePosition"),
    ("underline-thickness", "underlineThickness"),
    ("unicode-bidi", "unicodeBidi"),
    ("unicode-range", "unicodeRange"),
    ("units-per-em", "unitsPerEm"),
    ("v-alphabetic", "vAlphabetic"),
    ("v-hanging", "vHanging"),
    ("v-ideographic", "vIdeographic"),
    ("v-mathematical", "vMathematical"),
    ("vector-effect", "vectorEffect"),
    ("vert-adv-y", "vertAdvY"),
    ("vert-origin-x", "vertOriginX"),
    ("vert-origin-y", "vertOriginY"),
    ("word-spacing", "wordSpacing"),
    ("writing-mode", "writingMode"),
    ("x-height", "xHeight"),
    ("xlink:actuate", "xlinkActuate"),
    ("xlink:arcrole", "xlinkArcrole"),
    ("xlink:href", "xlinkHref"),
    ("xlink:role", "xlinkRole"),
    ("xlink:show", "xlinkShow"),
    ("xlink:title", "xlinkTitle"),
    ("xlink:type", "xlinkType"),
    ("xml:base", "xmlBase"),
    ("xml:lang", "xmlLang"),
    ("xml:space", "xmlSpace"),
];

const DOM_PROPERTY_NAMES_ONE_WORD: [&str; 190] = [
    // Global attributes - can be used on any HTML/DOM element
    // See https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
    "dir",
    "draggable",
    "hidden",
    "id",
    "lang",
    "nonce",
    "part",
    "slot",
    "style",
    "title",
    "translate",
    "inert",
    // Element specific attributes
    // See https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes (includes global attributes too)
    // To be considered if these should be added also to ATTRIBUTE_TAGS_MAP
    "accept",
    "action",
    "allow",
    "alt",
    "as",
    "async",
    "buffered",
    "capture",
    "challenge",
    "cite",
    "code",
    "cols",
    "content",
    "coords",
    "csp",
    "data",
    "decoding",
    "default",
    "defer",
    "disabled",
    "form",
    "headers",
    "height",
    "high",
    "href",
    "icon",
    "importance",
    "integrity",
    "kind",
    "label",
    "language",
    "loading",
    "list",
    "loop",
    "low",
    "manifest",
    "max",
    "media",
    "method",
    "min",
    "multiple",
    "muted",
    "name",
    "open",
    "optimum",
    "pattern",
    "ping",
    "placeholder",
    "poster",
    "preload",
    "profile",
    "rel",
    "required",
    "reversed",
    "role",
    "rows",
    "sandbox",
    "scope",
    "seamless",
    "selected",
    "shape",
    "size",
    "sizes",
    "span",
    "src",
    "start",
    "step",
    "summary",
    "target",
    "type",
    "value",
    "width",
    "wmode",
    "wrap",
    // SVG attributes
    // See https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute
    "accumulate",
    "additive",
    "alphabetic",
    "amplitude",
    "ascent",
    "azimuth",
    "bbox",
    "begin",
    "bias",
    "by",
    "clip",
    "color",
    "cursor",
    "cx",
    "cy",
    "d",
    "decelerate",
    "descent",
    "direction",
    "display",
    "divisor",
    "dur",
    "dx",
    "dy",
    "elevation",
    "end",
    "exponent",
    "fill",
    "filter",
    "format",
    "from",
    "fr",
    "fx",
    "fy",
    "g1",
    "g2",
    "hanging",
    "height",
    "hreflang",
    "ideographic",
    "in",
    "in2",
    "intercept",
    "k",
    "k1",
    "k2",
    "k3",
    "k4",
    "kerning",
    "local",
    "mask",
    "mode",
    "offset",
    "opacity",
    "operator",
    "order",
    "orient",
    "orientation",
    "origin",
    "overflow",
    "path",
    "ping",
    "points",
    "r",
    "radius",
    "rel",
    "restart",
    "result",
    "rotate",
    "rx",
    "ry",
    "scale",
    "seed",
    "slope",
    "spacing",
    "speed",
    "stemh",
    "stemv",
    "string",
    "stroke",
    "to",
    "transform",
    "u1",
    "u2",
    "unicode",
    "values",
    "version",
    "visibility",
    "widths",
    "x",
    "x1",
    "x2",
    "xmlns",
    "y",
    "y1",
    "y2",
    "z",
    // OpenGraph meta tag attributes
    "property",
    // React specific attributes
    "ref",
    "key",
    "children",
    // Non-standard
    "results",
    "security",
    // Video specific
    "controls",
];

const DOM_PROPERTY_NAMES_TWO_WORDS: [&str; 359] = [
    // Global attributes - can be used on any HTML/DOM element
    // See https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes
    "accessKey",
    "autoCapitalize",
    "autoFocus",
    "contentEditable",
    "enterKeyHint",
    "exportParts",
    "inputMode",
    "itemID",
    "itemRef",
    "itemProp",
    "itemScope",
    "itemType",
    "spellCheck",
    "tabIndex",
    // Element specific attributes
    // See https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes (includes global attributes too)
    // To be considered if these should be added also to ATTRIBUTE_TAGS_MAP
    "acceptCharset",
    "autoComplete",
    "autoPlay",
    "border",
    "cellPadding",
    "cellSpacing",
    "classID",
    "codeBase",
    "colSpan",
    "contextMenu",
    "dateTime",
    "encType",
    "formAction",
    "formEncType",
    "formMethod",
    "formNoValidate",
    "formTarget",
    "frameBorder",
    "hrefLang",
    "httpEquiv",
    "imageSizes",
    "imageSrcSet",
    "isMap",
    "keyParams",
    "keyType",
    "marginHeight",
    "marginWidth",
    "maxLength",
    "mediaGroup",
    "minLength",
    "noValidate",
    "onAnimationEnd",
    "onAnimationIteration",
    "onAnimationStart",
    "onBlur",
    "onChange",
    "onClick",
    "onContextMenu",
    "onCopy",
    "onCompositionEnd",
    "onCompositionStart",
    "onCompositionUpdate",
    "onCut",
    "onDoubleClick",
    "onDrag",
    "onDragEnd",
    "onDragEnter",
    "onDragExit",
    "onDragLeave",
    "onError",
    "onFocus",
    "onInput",
    "onKeyDown",
    "onKeyPress",
    "onKeyUp",
    "onLoad",
    "onWheel",
    "onDragOver",
    "onDragStart",
    "onDrop",
    "onMouseDown",
    "onMouseEnter",
    "onMouseLeave",
    "onMouseMove",
    "onMouseOut",
    "onMouseOver",
    "onMouseUp",
    "onPaste",
    "onScroll",
    "onSelect",
    "onSubmit",
    "onToggle",
    "onTransitionEnd",
    "radioGroup",
    "readOnly",
    "referrerPolicy",
    "rowSpan",
    "srcDoc",
    "srcLang",
    "srcSet",
    "useMap",
    "fetchPriority",
    // SVG attributes
    // See https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute
    "crossOrigin",
    "accentHeight",
    "alignmentBaseline",
    "arabicForm",
    "attributeName",
    "attributeType",
    "baseFrequency",
    "baselineShift",
    "baseProfile",
    "calcMode",
    "capHeight",
    "clipPathUnits",
    "clipPath",
    "clipRule",
    "colorInterpolation",
    "colorInterpolationFilters",
    "colorProfile",
    "colorRendering",
    "contentScriptType",
    "contentStyleType",
    "diffuseConstant",
    "dominantBaseline",
    "edgeMode",
    "enableBackground",
    "fillOpacity",
    "fillRule",
    "filterRes",
    "filterUnits",
    "floodColor",
    "floodOpacity",
    "fontFamily",
    "fontSize",
    "fontSizeAdjust",
    "fontStretch",
    "fontStyle",
    "fontVariant",
    "fontWeight",
    "glyphName",
    "glyphOrientationHorizontal",
    "glyphOrientationVertical",
    "glyphRef",
    "gradientTransform",
    "gradientUnits",
    "horizAdvX",
    "horizOriginX",
    "imageRendering",
    "kernelMatrix",
    "kernelUnitLength",
    "keyPoints",
    "keySplines",
    "keyTimes",
    "lengthAdjust",
    "letterSpacing",
    "lightingColor",
    "limitingConeAngle",
    "markerEnd",
    "markerMid",
    "markerStart",
    "markerHeight",
    "markerUnits",
    "markerWidth",
    "maskContentUnits",
    "maskUnits",
    "mathematical",
    "numOctaves",
    "overlinePosition",
    "overlineThickness",
    "panose1",
    "paintOrder",
    "pathLength",
    "patternContentUnits",
    "patternTransform",
    "patternUnits",
    "pointerEvents",
    "pointsAtX",
    "pointsAtY",
    "pointsAtZ",
    "preserveAlpha",
    "preserveAspectRatio",
    "primitiveUnits",
    "referrerPolicy",
    "refX",
    "refY",
    "rendering-intent",
    "repeatCount",
    "repeatDur",
    "requiredExtensions",
    "requiredFeatures",
    "shapeRendering",
    "specularConstant",
    "specularExponent",
    "spreadMethod",
    "startOffset",
    "stdDeviation",
    "stitchTiles",
    "stopColor",
    "stopOpacity",
    "strikethroughPosition",
    "strikethroughThickness",
    "strokeDasharray",
    "strokeDashoffset",
    "strokeLinecap",
    "strokeLinejoin",
    "strokeMiterlimit",
    "strokeOpacity",
    "strokeWidth",
    "surfaceScale",
    "systemLanguage",
    "tableValues",
    "targetX",
    "targetY",
    "textAnchor",
    "textDecoration",
    "textRendering",
    "textLength",
    "transformOrigin",
    "underlinePosition",
    "underlineThickness",
    "unicodeBidi",
    "unicodeRange",
    "unitsPerEm",
    "vAlphabetic",
    "vHanging",
    "vIdeographic",
    "vMathematical",
    "vectorEffect",
    "vertAdvY",
    "vertOriginX",
    "vertOriginY",
    "viewBox",
    "viewTarget",
    "wordSpacing",
    "writingMode",
    "xHeight",
    "xChannelSelector",
    "xlinkActuate",
    "xlinkArcrole",
    "xlinkHref",
    "xlinkRole",
    "xlinkShow",
    "xlinkTitle",
    "xlinkType",
    "xmlBase",
    "xmlLang",
    "xmlnsXlink",
    "xmlSpace",
    "yChannelSelector",
    "zoomAndPan",
    // Safari/Apple specific, no listing available
    "autoCorrect", // https://stackoverflow.com/questions/47985384/html-autocorrect-for-text-input-is-not-working
    "autoSave", // https://stackoverflow.com/questions/25456396/what-is-autosave-attribute-supposed-to-do-how-do-i-use-it
    // React specific attributes https://reactjs.org/docs/dom-elements.html#differences-in-attributes
    "className",
    "dangerouslySetInnerHTML",
    "defaultValue",
    "defaultChecked",
    "htmlFor",
    // Events' capture events
    "onBeforeInput",
    "onChange",
    "onInvalid",
    "onReset",
    "onTouchCancel",
    "onTouchEnd",
    "onTouchMove",
    "onTouchStart",
    "suppressContentEditableWarning",
    "suppressHydrationWarning",
    "onAbort",
    "onCanPlay",
    "onCanPlayThrough",
    "onDurationChange",
    "onEmptied",
    "onEncrypted",
    "onEnded",
    "onLoadedData",
    "onLoadedMetadata",
    "onLoadStart",
    "onPause",
    "onPlay",
    "onPlaying",
    "onProgress",
    "onRateChange",
    "onResize",
    "onSeeked",
    "onSeeking",
    "onStalled",
    "onSuspend",
    "onTimeUpdate",
    "onVolumeChange",
    "onWaiting",
    "onCopyCapture",
    "onCutCapture",
    "onPasteCapture",
    "onCompositionEndCapture",
    "onCompositionStartCapture",
    "onCompositionUpdateCapture",
    "onFocusCapture",
    "onBlurCapture",
    "onChangeCapture",
    "onBeforeInputCapture",
    "onInputCapture",
    "onResetCapture",
    "onSubmitCapture",
    "onInvalidCapture",
    "onLoadCapture",
    "onErrorCapture",
    "onKeyDownCapture",
    "onKeyPressCapture",
    "onKeyUpCapture",
    "onAbortCapture",
    "onCanPlayCapture",
    "onCanPlayThroughCapture",
    "onDurationChangeCapture",
    "onEmptiedCapture",
    "onEncryptedCapture",
    "onEndedCapture",
    "onLoadedDataCapture",
    "onLoadedMetadataCapture",
    "onLoadStartCapture",
    "onPauseCapture",
    "onPlayCapture",
    "onPlayingCapture",
    "onProgressCapture",
    "onRateChangeCapture",
    "onSeekedCapture",
    "onSeekingCapture",
    "onStalledCapture",
    "onSuspendCapture",
    "onTimeUpdateCapture",
    "onVolumeChangeCapture",
    "onWaitingCapture",
    "onSelectCapture",
    "onTouchCancelCapture",
    "onTouchEndCapture",
    "onTouchMoveCapture",
    "onTouchStartCapture",
    "onScrollCapture",
    "onWheelCapture",
    "onAnimationEndCapture",
    "onAnimationIteration",
    "onAnimationStartCapture",
    "onTransitionEndCapture",
    "onAuxClick",
    "onAuxClickCapture",
    "onClickCapture",
    "onContextMenuCapture",
    "onDoubleClickCapture",
    "onDragCapture",
    "onDragEndCapture",
    "onDragEnterCapture",
    "onDragExitCapture",
    "onDragLeaveCapture",
    "onDragOverCapture",
    "onDragStartCapture",
    "onDropCapture",
    "onMouseDown",
    "onMouseDownCapture",
    "onMouseMoveCapture",
    "onMouseOutCapture",
    "onMouseOverCapture",
    "onMouseUpCapture",
    // Video specific
    "autoPictureInPicture",
    "controlsList",
    "disablePictureInPicture",
    "disableRemotePlayback",
];

fn normalize_attribute_case(name: &str) -> &str {
    DOM_PROPERTIES_IGNORE_CASE
        .iter()
        .find(|element| element.to_lowercase() == name.to_lowercase())
        .copied()
        .unwrap_or(name)
}

fn is_valid_data_attribute(name: &str) -> bool {
    let re_data_xml = Regex::new("(?i)^data-xml").unwrap();
    let re_data_general = Regex::new(r"^data-[^:]*$").unwrap();

    !re_data_xml.is_match(name) && re_data_general.is_match(name)
}

fn is_valid_aria_attribute(name: &str) -> bool {
    ARIA_PROPERTIES.iter().any(|&element| element == name)
}

fn get_dom_property_names() -> Vec<&'static str> {
    let all_dom_property_names: Vec<&str> = DOM_PROPERTY_NAMES_TWO_WORDS
        .iter()
        .chain(DOM_PROPERTY_NAMES_ONE_WORD.iter())
        .copied()
        .collect();

    all_dom_property_names
}

fn is_valid_html_tag_in_jsx(node: &AnyJsxElement) -> Option<bool> {
    let tag_convention = Regex::new(r"^[a-z][^-]*$").unwrap();

    let tag_name = match node {
        AnyJsxElement::JsxOpeningElement(opening_element) => {
            opening_element.name().ok()?.name_value_token().ok()?
        }
        AnyJsxElement::JsxSelfClosingElement(self_closing_element) => {
            self_closing_element.name().ok()?.name_value_token().ok()?
        }
    };

    if tag_convention.is_match(tag_name.text_trimmed()) {
        return Some(
            !node
                .attributes()
                .iter()
                .any(|attr| attr.to_trimmed_text() == "is"),
        );
    }

    Some(false)
}

fn get_tag_name(element: &AnyJsxElement) -> Option<String> {
    match element {
        AnyJsxElement::JsxOpeningElement(opening_element) => {
            let name = opening_element.name().ok()?;
            return Some(name.to_trimmed_string());
        }
        AnyJsxElement::JsxSelfClosingElement(self_closing_element) => {
            let name = self_closing_element.name().ok()?;
            return Some(name.to_trimmed_string());
        }
    };
}

fn tag_name_has_dot(node: &AnyJsxElement) -> Option<bool> {
    match node {
        AnyJsxElement::JsxOpeningElement(element) => Some(matches!(
            element.name().ok()?,
            AnyJsxElementName::JsxMemberName(_)
        )),
        AnyJsxElement::JsxSelfClosingElement(element) => Some(matches!(
            element.name().ok()?,
            AnyJsxElementName::JsxMemberName(_)
        )),
    }
}
pub enum NoUnknownPropertyDiagnostic {
    UnknownProp {
        name: String,
    },
    UnknownPropWithStandardName {
        name: String,
        standard_name: String,
    },
    InvalidPropOnTag {
        name: String,
        tag_name: String,
        allowed_tags: String,
    },
    DataLowercaseRequired {
        name: String,
        lowercase_name: String,
    },
}

fn get_standard_name(name: &str) -> Option<&'static str> {
    if let Ok(idx) = DOM_ATTRIBUTE_NAMES.binary_search_by_key(&name, |&(key, _)| key) {
        return Some(DOM_ATTRIBUTE_NAMES[idx].1);
    }

    if let Ok(idx) = SVGDOM_ATTRIBUTE_NAMES.binary_search_by_key(&name, |&(key, _)| key) {
        return Some(SVGDOM_ATTRIBUTE_NAMES[idx].1);
    }

    let names = get_dom_property_names();

    return names
        .iter()
        .find(|&&element| element.to_lowercase() == name.to_lowercase())
        .copied();
}

impl Rule for NoUnknownProperty {
    type Query = Ast<JsxAttribute>;
    type State = (TextRange, NoUnknownPropertyDiagnostic);
    type Signals = Option<Self::State>;
    type Options = NoUnknownPropertyOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let node_name = node.name_value_token().ok()?;

        let name = normalize_attribute_case(node_name.text_trimmed());
        let parent = node.syntax().parent()?.parent()?;
        let element = AnyJsxElement::cast_ref(&parent)?;

        if tag_name_has_dot(&element)? {
            return None;
        }

        if is_valid_data_attribute(name) {
            if name.to_lowercase() != name {
                return Some((
                    node.range(),
                    NoUnknownPropertyDiagnostic::DataLowercaseRequired {
                        name: name.to_string(),
                        lowercase_name: name.to_lowercase(),
                    },
                ));
            }
            return None;
        }

        if is_valid_aria_attribute(name) {
            return None;
        }

        let tag_name = get_tag_name(&element)?;

        if tag_name == "fbt" || tag_name == "fbs" {
            return None;
        }

        if !is_valid_html_tag_in_jsx(&element)? {
            return None;
        }

        let allowed_tags = ATTRIBUTE_TAGS_MAP
            .binary_search_by_key(&name, |&(key, _)| key)
            .ok()
            .map(|idx| ATTRIBUTE_TAGS_MAP[idx].1);

        if let Some(allowed_tags) = allowed_tags {
            if !allowed_tags.contains(&&tag_name.as_str()) {
                return Some((
                    node.range(),
                    NoUnknownPropertyDiagnostic::InvalidPropOnTag {
                        name: name.to_string(),
                        tag_name: tag_name.to_string(),
                        allowed_tags: allowed_tags.join(","),
                    },
                ));
            }
        }

        if let Some(standard_name) = get_standard_name(name) {
            if standard_name != name {
                return Some((
                    node.range(),
                    NoUnknownPropertyDiagnostic::UnknownPropWithStandardName {
                        name: name.to_string(),
                        standard_name: standard_name.to_string(),
                    },
                ));
            }
        }

        Some((
            node.range(),
            NoUnknownPropertyDiagnostic::UnknownProp {
                name: name.to_string(),
            },
        ))
    }

    fn diagnostic(_ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let (range, diagnostic_kind) = _state;
        match diagnostic_kind {
            NoUnknownPropertyDiagnostic::UnknownProp { name } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "Unknown property '" {name} "' found"
                    },
                )
                .note(markup! {
                    "This note will give you more information."
                }),
            ),
            NoUnknownPropertyDiagnostic::UnknownPropWithStandardName {
                name,
                standard_name,
            } => {
                let mut diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "Invalid DOM property '" {name} "'. Did you mean '" {standard_name} "'?"
                    },
                );

                diagnostic = diagnostic.note(markup! {
                       "React uses camelCased props like '" {standard_name} "' instead of HTML attribute names."
                   });

                Some(diagnostic)
            }
            NoUnknownPropertyDiagnostic::InvalidPropOnTag {
                name,
                tag_name,
                allowed_tags,
            } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    *range,
                    markup! {
                        "Property '" {name} "' is not valid on a <" {tag_name} "> element."
                    },
                )
                .note(markup! {
                    "This attribute is only allowed on: " {allowed_tags}
                }),
            ),
            NoUnknownPropertyDiagnostic::DataLowercaseRequired {
                name,
                lowercase_name,
            } => Some(RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "data-* attributes should be lowercase. Use '" {lowercase_name} "' instead of '" {name} "'."
                },
            )),
        }
    }
}
