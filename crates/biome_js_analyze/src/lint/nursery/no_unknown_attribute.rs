use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_analyze::{RuleDomain, RuleSource};
use biome_console::markup;
use biome_js_syntax::AnyJsxAttributeName;
use biome_js_syntax::{AnyJsxElementName, JsxAttribute, jsx_ext::AnyJsxElement};
use biome_package::PackageJson;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_unknown_attribute::NoUnknownAttributeOptions;
use camino::Utf8PathBuf;
use rustc_hash::FxHashMap;
use std::sync::{Arc, LazyLock};

use crate::services::manifest::Manifest;

declare_lint_rule! {
    /// Disallow unknown DOM properties.
    ///
    /// In JSX, most DOM properties and attributes should be camelCased to be consistent with standard JavaScript style.
    /// This can be a possible source of error if you are used to writing plain HTML.
    /// Only `data-*` and `aria-*` attributes are allowed to use hyphens and lowercase letters in JSX.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div allowTransparency="true" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div onclick={() => {}} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div for="bar" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="foo" />
    /// ```
    ///
    /// ```jsx
    /// <div onClick={() => {}} />
    /// ```
    ///
    /// ```jsx
    /// <div htmlFor="bar" />
    /// ```
    ///
    /// ```jsx
    /// <div data-foo="bar" />
    /// ```
    ///
    /// ```jsx
    /// <div aria-label="Close" />
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignore`
    ///
    /// An array of property and attribute names to ignore during validation.
    ///
    /// ```json
    /// {
    ///   "noUnknownAttribute": {
    ///     "options": {
    ///       "ignore": ["custom-attribute", "non-standard-prop"]
    ///     }
    ///   }
    /// }
    /// ```
    pub NoUnknownAttribute {
        version: "2.3.3",
        name: "noUnknownAttribute",
        language: "jsx",
        domains: &[RuleDomain::React],
        sources: &[
            RuleSource::EslintReact("no-unknown-property").same(),
        ],
        recommended: false,
    }
}

/**
 * Popover API properties added in React 19
 */
const POPOVER_API_PROPS: &[&str] = &[
    "onBeforeToggle",
    "popover",
    "popoverTarget",
    "popoverTargetAction",
];

const POPOVER_API_PROPS_LOWERCASE: &[&str] = &[
    "onbeforetoggle",
    "popover",
    "popovertarget",
    "popovertargetaction",
];
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
    ("closedby", &["dialog"]),
    ("controls", &["audio", "video"]),
    ("controlsList", &["audio", "video"]),
    (
        "crossOrigin",
        &["script", "img", "video", "audio", "link", "image"],
    ),
    ("disablePictureInPicture", &["video"]),
    ("disableRemotePlayback", &["audio", "video"]),
    ("displaystyle", &["math"]),
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

static ATTRIBUTE_TAGS_LOOKUP: LazyLock<FxHashMap<&'static str, &'static [&'static str]>> =
    LazyLock::new(|| ATTRIBUTE_TAGS_MAP.iter().copied().collect());

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
    ("accept-charset", "acceptCharset"),
    ("accent-height", "accentHeight"),
    ("alignment-baseline", "alignmentBaseline"),
    ("arabic-form", "arabicForm"),
    ("baseline-shift", "baselineShift"),
    ("cap-height", "capHeight"),
    ("class", "className"),
    ("clip-path", "clipPath"),
    ("clip-rule", "clipRule"),
    ("color-interpolation", "colorInterpolation"),
    ("color-interpolation-filters", "colorInterpolationFilters"),
    ("color-profile", "colorProfile"),
    ("color-rendering", "colorRendering"),
    ("crossorigin", "crossOrigin"),
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
    ("for", "htmlFor"),
    ("glyph-name", "glyphName"),
    ("glyph-orientation-horizontal", "glyphOrientationHorizontal"),
    ("glyph-orientation-vertical", "glyphOrientationVertical"),
    ("horiz-adv-x", "horizAdvX"),
    ("horiz-origin-x", "horizOriginX"),
    ("http-equiv", "httpEquiv"),
    ("image-rendering", "imageRendering"),
    ("letter-spacing", "letterSpacing"),
    ("lighting-color", "lightingColor"),
    ("marker-end", "markerEnd"),
    ("marker-mid", "markerMid"),
    ("marker-start", "markerStart"),
    ("nomodule", "noModule"),
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

static DOM_ATTRIBUTE_LOOKUP: LazyLock<FxHashMap<&'static str, &'static str>> =
    LazyLock::new(|| DOM_ATTRIBUTE_NAMES.iter().copied().collect());

const DOM_PROPERTY_NAMES: &[&str] = &[
    "accentHeight",
    "accept",
    "acceptCharset",
    "accessKey",
    "accumulate",
    "action",
    "additive",
    "alignmentBaseline",
    "allow",
    "alphabetic",
    "alt",
    "amplitude",
    "arabicForm",
    "as",
    "ascent",
    "async",
    "attributeName",
    "attributeType",
    "autoCapitalize",
    "autoComplete",
    "autoCorrect",
    "autoFocus",
    "autoPictureInPicture",
    "autoPlay",
    "autoSave",
    "azimuth",
    "baseFrequency",
    "baselineShift",
    "baseProfile",
    "bbox",
    "begin",
    "bias",
    "border",
    "buffered",
    "by",
    "calcMode",
    "capHeight",
    "capture",
    "cellPadding",
    "cellSpacing",
    "challenge",
    "children",
    "cite",
    "classID",
    "className",
    "clip",
    "clipPath",
    "clipPathUnits",
    "clipRule",
    "code",
    "codeBase",
    "color",
    "colorInterpolation",
    "colorInterpolationFilters",
    "colorProfile",
    "colorRendering",
    "cols",
    "colSpan",
    "content",
    "contentEditable",
    "contentScriptType",
    "contentStyleType",
    "contextMenu",
    "controls",
    "controlsList",
    "coords",
    "crossOrigin",
    "csp",
    "cursor",
    "cx",
    "cy",
    "d",
    "dangerouslySetInnerHTML",
    "data",
    "dateTime",
    "decelerate",
    "decoding",
    "default",
    "defaultChecked",
    "defaultValue",
    "defer",
    "descent",
    "diffuseConstant",
    "dir",
    "direction",
    "disabled",
    "disablePictureInPicture",
    "disableRemotePlayback",
    "display",
    "divisor",
    "dominantBaseline",
    "draggable",
    "dur",
    "dx",
    "dy",
    "edgeMode",
    "elevation",
    "enableBackground",
    "encType",
    "end",
    "enterKeyHint",
    "exponent",
    "exportParts",
    "fetchPriority",
    "fill",
    "fillOpacity",
    "fillRule",
    "filter",
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
    "form",
    "formAction",
    "format",
    "formEncType",
    "formMethod",
    "formNoValidate",
    "formTarget",
    "fr",
    "frameBorder",
    "from",
    "fx",
    "fy",
    "g1",
    "g2",
    "glyphName",
    "glyphOrientationHorizontal",
    "glyphOrientationVertical",
    "glyphRef",
    "gradientTransform",
    "gradientUnits",
    "hanging",
    "headers",
    "height",
    "height",
    "hidden",
    "high",
    "horizAdvX",
    "horizOriginX",
    "href",
    "hreflang",
    "hrefLang",
    "htmlFor",
    "httpEquiv",
    "icon",
    "id",
    "ideographic",
    "imageRendering",
    "imageSizes",
    "imageSrcSet",
    "importance",
    "in",
    "in2",
    "inert",
    "inputMode",
    "integrity",
    "intercept",
    "isMap",
    "itemID",
    "itemProp",
    "itemRef",
    "itemScope",
    "itemType",
    "k",
    "k1",
    "k2",
    "k3",
    "k4",
    "kernelMatrix",
    "kernelUnitLength",
    "kerning",
    "key",
    "keyParams",
    "keyPoints",
    "keySplines",
    "keyTimes",
    "keyType",
    "kind",
    "label",
    "lang",
    "language",
    "lengthAdjust",
    "letterSpacing",
    "lightingColor",
    "limitingConeAngle",
    "list",
    "loading",
    "local",
    "loop",
    "low",
    "manifest",
    "marginHeight",
    "marginWidth",
    "markerEnd",
    "markerHeight",
    "markerMid",
    "markerStart",
    "markerUnits",
    "markerWidth",
    "mask",
    "maskContentUnits",
    "maskUnits",
    "mathematical",
    "max",
    "maxLength",
    "media",
    "mediaGroup",
    "method",
    "min",
    "minLength",
    "mode",
    "multiple",
    "muted",
    "name",
    "nonce",
    "noValidate",
    "numOctaves",
    "offset",
    "onAbort",
    "onAbortCapture",
    "onAnimationEnd",
    "onAnimationEndCapture",
    "onAnimationIteration",
    "onAnimationIteration",
    "onAnimationStart",
    "onAnimationStartCapture",
    "onAuxClick",
    "onAuxClickCapture",
    "onBeforeInput",
    "onBeforeInputCapture",
    "onBlur",
    "onBlurCapture",
    "onCanPlay",
    "onCanPlayCapture",
    "onCanPlayThrough",
    "onCanPlayThroughCapture",
    "onChange",
    "onChange",
    "onChangeCapture",
    "onClick",
    "onClickCapture",
    "onCompositionEnd",
    "onCompositionEndCapture",
    "onCompositionStart",
    "onCompositionStartCapture",
    "onCompositionUpdate",
    "onCompositionUpdateCapture",
    "onContextMenu",
    "onContextMenuCapture",
    "onCopy",
    "onCopyCapture",
    "onCut",
    "onCutCapture",
    "onDoubleClick",
    "onDoubleClickCapture",
    "onDrag",
    "onDragCapture",
    "onDragEnd",
    "onDragEndCapture",
    "onDragEnter",
    "onDragEnterCapture",
    "onDragExit",
    "onDragExitCapture",
    "onDragLeave",
    "onDragLeaveCapture",
    "onDragOver",
    "onDragOverCapture",
    "onDragStart",
    "onDragStartCapture",
    "onDrop",
    "onDropCapture",
    "onDurationChange",
    "onDurationChangeCapture",
    "onEmptied",
    "onEmptiedCapture",
    "onEncrypted",
    "onEncryptedCapture",
    "onEnded",
    "onEndedCapture",
    "onError",
    "onErrorCapture",
    "onFocus",
    "onFocusCapture",
    "onGotPointerCapture",
    "onGotPointerCaptureCapture",
    "onInput",
    "onInputCapture",
    "onInvalid",
    "onInvalidCapture",
    "onKeyDown",
    "onKeyDownCapture",
    "onKeyPress",
    "onKeyPressCapture",
    "onKeyUp",
    "onKeyUpCapture",
    "onLoad",
    "onLoadCapture",
    "onLoadedData",
    "onLoadedDataCapture",
    "onLoadedMetadata",
    "onLoadedMetadataCapture",
    "onLoadStart",
    "onLoadStartCapture",
    "onLostPointerCapture",
    "onLostPointerCapture",
    "onLostPointerCaptureCapture",
    "onMouseDown",
    "onMouseDown",
    "onMouseDownCapture",
    "onMouseEnter",
    "onMouseLeave",
    "onMouseMove",
    "onMouseMoveCapture",
    "onMouseOut",
    "onMouseOutCapture",
    "onMouseOver",
    "onMouseOverCapture",
    "onMouseUp",
    "onMouseUpCapture",
    "onPaste",
    "onPasteCapture",
    "onPause",
    "onPauseCapture",
    "onPlay",
    "onPlayCapture",
    "onPlaying",
    "onPlayingCapture",
    "onPointerCancel",
    "onPointerCancelCapture",
    "onPointerDown",
    "onPointerDownCapture",
    "onPointerEnter",
    "onPointerEnterCapture",
    "onPointerLeave",
    "onPointerLeaveCapture",
    "onPointerMove",
    "onPointerMoveCapture",
    "onPointerOut",
    "onPointerOutCapture",
    "onPointerOver",
    "onPointerOverCapture",
    "onPointerUp",
    "onPointerUpCapture",
    "onProgress",
    "onProgressCapture",
    "onRateChange",
    "onRateChangeCapture",
    "onReset",
    "onResetCapture",
    "onResize",
    "onScroll",
    "onScrollCapture",
    "onSeeked",
    "onSeekedCapture",
    "onSeeking",
    "onSeekingCapture",
    "onSelect",
    "onSelectCapture",
    "onStalled",
    "onStalledCapture",
    "onSubmit",
    "onSubmitCapture",
    "onSuspend",
    "onSuspendCapture",
    "onTimeUpdate",
    "onTimeUpdateCapture",
    "onToggle",
    "onTouchCancel",
    "onTouchCancelCapture",
    "onTouchEnd",
    "onTouchEndCapture",
    "onTouchMove",
    "onTouchMoveCapture",
    "onTouchStart",
    "onTouchStartCapture",
    "onTransitionEnd",
    "onTransitionEndCapture",
    "onVolumeChange",
    "onVolumeChangeCapture",
    "onWaiting",
    "onWaitingCapture",
    "onWheel",
    "onWheelCapture",
    "opacity",
    "open",
    "operator",
    "optimum",
    "order",
    "orient",
    "orientation",
    "origin",
    "overflow",
    "overlinePosition",
    "overlineThickness",
    "paintOrder",
    "panose1",
    "part",
    "path",
    "pathLength",
    "pattern",
    "patternContentUnits",
    "patternTransform",
    "patternUnits",
    "ping",
    "ping",
    "placeholder",
    "pointerEvents",
    "points",
    "pointsAtX",
    "pointsAtY",
    "pointsAtZ",
    "poster",
    "preload",
    "preserveAlpha",
    "preserveAspectRatio",
    "primitiveUnits",
    "profile",
    "property",
    "r",
    "radioGroup",
    "radius",
    "readOnly",
    "ref",
    "referrerPolicy",
    "referrerPolicy",
    "refX",
    "refY",
    "rel",
    "rel",
    "rendering-intent",
    "repeatCount",
    "repeatDur",
    "required",
    "requiredExtensions",
    "requiredFeatures",
    "restart",
    "result",
    "results",
    "reversed",
    "role",
    "rotate",
    "rows",
    "rowSpan",
    "rx",
    "ry",
    "sandbox",
    "scale",
    "scope",
    "seamless",
    "security",
    "seed",
    "selected",
    "shape",
    "shapeRendering",
    "size",
    "sizes",
    "slope",
    "slot",
    "spacing",
    "span",
    "specularConstant",
    "specularExponent",
    "speed",
    "spellCheck",
    "spreadMethod",
    "src",
    "srcDoc",
    "srcLang",
    "srcSet",
    "start",
    "startOffset",
    "stdDeviation",
    "stemh",
    "stemv",
    "step",
    "stitchTiles",
    "stopColor",
    "stopOpacity",
    "strikethroughPosition",
    "strikethroughThickness",
    "string",
    "stroke",
    "strokeDasharray",
    "strokeDashoffset",
    "strokeLinecap",
    "strokeLinejoin",
    "strokeMiterlimit",
    "strokeOpacity",
    "strokeWidth",
    "style",
    "summary",
    "suppressContentEditableWarning",
    "suppressHydrationWarning",
    "surfaceScale",
    "systemLanguage",
    "tabIndex",
    "tableValues",
    "target",
    "targetX",
    "targetY",
    "textAnchor",
    "textDecoration",
    "textLength",
    "textRendering",
    "title",
    "to",
    "transform",
    "transformOrigin",
    "translate",
    "type",
    "u1",
    "u2",
    "underlinePosition",
    "underlineThickness",
    "unicode",
    "unicodeBidi",
    "unicodeRange",
    "unitsPerEm",
    "useMap",
    "vAlphabetic",
    "value",
    "values",
    "vectorEffect",
    "version",
    "vertAdvY",
    "vertOriginX",
    "vertOriginY",
    "vHanging",
    "vIdeographic",
    "viewBox",
    "viewTarget",
    "visibility",
    "vMathematical",
    "width",
    "widths",
    "wmode",
    "wordSpacing",
    "wrap",
    "writingMode",
    "x",
    "x1",
    "x2",
    "xChannelSelector",
    "xHeight",
    "xlinkActuate",
    "xlinkArcrole",
    "xlinkHref",
    "xlinkRole",
    "xlinkShow",
    "xlinkTitle",
    "xlinkType",
    "xmlBase",
    "xmlLang",
    "xmlns",
    "xmlnsXlink",
    "xmlSpace",
    "y",
    "y1",
    "y2",
    "yChannelSelector",
    "z",
    "zoomAndPan",
];

fn is_valid_data_attribute(name: &str) -> bool {
    use biome_string_case::StrOnlyExtension;
    if !name.starts_with("data-") {
        return false;
    }

    if name.to_lowercase_cow().starts_with("data-xml") {
        return false;
    }

    let data_name = &name["data-".len()..];

    if data_name.is_empty() {
        return false;
    }

    data_name.chars().all(|c| c != ':')
}

fn is_valid_aria_attribute(name: &str) -> bool {
    ARIA_PROPERTIES.contains(&name)
}

fn is_valid_html_tag_in_jsx(node: &AnyJsxElement, tag_name: &str) -> bool {
    let matches_tag_convention = tag_name.char_indices().all(|(i, c)| {
        if i == 0 {
            c.is_ascii_lowercase()
        } else {
            c != '-'
        }
    });

    if matches_tag_convention {
        return node.attributes().find_by_name("is").is_none();
    }

    false
}

fn tag_name_has_dot(node: &AnyJsxElement) -> Option<bool> {
    Some(matches!(
        node.name().ok()?,
        AnyJsxElementName::JsxMemberName(_)
    ))
}

pub enum NoUnknownAttributeState {
    UnknownProp {
        name: Box<str>,
    },
    UnknownPropWithStandardName {
        name: Box<str>,
        standard_name: Box<str>,
    },
    InvalidPropOnTag {
        name: Box<str>,
        tag_name: TokenText,
        allowed_tags: &'static [&'static str],
    },
}

fn get_standard_name(ctx: &RuleContext<NoUnknownAttribute>, name: &str) -> Option<&'static str> {
    if let Some(&standard_name) = DOM_ATTRIBUTE_LOOKUP.get(name) {
        return Some(standard_name);
    }
    let is_react_19_or_later = ctx
        .get_service::<Option<(Utf8PathBuf, Arc<PackageJson>)>>()
        .and_then(|manifest| {
            manifest
                .as_ref()
                .map(|(_, package_json)| package_json.matches_dependency("react", ">=19.0.0"))
        })
        .unwrap_or(false);

    if is_react_19_or_later {
        if let Some(&prop) = POPOVER_API_PROPS
            .iter()
            .find(|&&element| element.eq_ignore_ascii_case(name))
        {
            return Some(prop);
        }
    } else if let Some(&prop) = POPOVER_API_PROPS_LOWERCASE
        .iter()
        .find(|&&element| element.eq_ignore_ascii_case(name))
    {
        return Some(prop);
    }

    DOM_PROPERTY_NAMES
        .iter()
        .find(|&&element| element.eq_ignore_ascii_case(name))
        .copied()
}

impl Rule for NoUnknownAttribute {
    type Query = Manifest<JsxAttribute>;
    type State = NoUnknownAttributeState;
    type Signals = Option<Self::State>;
    type Options = NoUnknownAttributeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let options = ctx.options();

        let node_name = match node.name().ok()? {
            AnyJsxAttributeName::JsxName(name) => name.syntax().text_trimmed(),
            AnyJsxAttributeName::JsxNamespaceName(name) => name.syntax().text_trimmed(),
        };

        let node_name = node_name.into_text();

        if options
            .ignore
            .iter()
            .flatten()
            .any(|ignored| ignored.as_ref() == node_name.text())
        {
            return None;
        }
        let name = if let Some(element) = DOM_PROPERTIES_IGNORE_CASE
            .iter()
            .find(|element| element.eq_ignore_ascii_case(&node_name))
        {
            element
        } else {
            &node_name.text()
        };

        let parent = node.syntax().parent()?.parent()?;
        let element = AnyJsxElement::cast_ref(&parent)?;

        // Ignore tags like <Foo.bar />
        if tag_name_has_dot(&element)? {
            return None;
        }

        // Handle data-* attributes
        if is_valid_data_attribute(name) {
            return None;
        }

        // Handle aria-* attributes
        if is_valid_aria_attribute(name) {
            return None;
        }

        let tag_name = element.name_value_token().ok()?.token_text_trimmed();

        // Special case for fbt/fbs nodes
        if tag_name == "fbt" || tag_name == "fbs" {
            return None;
        }

        // Only validate HTML/DOM elements, not React components
        if !is_valid_html_tag_in_jsx(&element, &tag_name) {
            return None;
        }

        let allowed_tags = ATTRIBUTE_TAGS_LOOKUP.get(name);

        if let Some(allowed_tags) = allowed_tags {
            if !allowed_tags.contains(&tag_name.trim()) {
                return Some(NoUnknownAttributeState::InvalidPropOnTag {
                    name: (*name).into(),
                    tag_name,
                    allowed_tags,
                });
            }
            return None;
        }

        if let Some(standard_name) = get_standard_name(ctx, name) {
            if standard_name != *name {
                return Some(NoUnknownAttributeState::UnknownPropWithStandardName {
                    name: (*name).into(),
                    standard_name: standard_name.into(),
                });
            }
            return None;
        }

        Some(NoUnknownAttributeState::UnknownProp {
            name: (*name).into(),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        match state {
            NoUnknownAttributeState::UnknownProp { name } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "The property '"{name}"' is not a valid DOM attribute."
                    },
                )
                .note(markup! {
                    "This property is not recognized as a valid HTML/DOM attribute or React prop."
                })
                .note(markup! {
                    "Check the spelling or consider using a valid data-* attribute for custom properties."
                }),
            ),
            NoUnknownAttributeState::UnknownPropWithStandardName {
                name,
                standard_name,
            } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "Property '"{name}"' is not a valid React prop name."
                    },
                )
                .note(markup! {
                    "React uses camelCased props, while HTML uses kebab-cased attributes."
                })
                .note(markup! {
                        "Use '"{standard_name}"' instead of '"{name}"' for React components."
                }),
            ),
            NoUnknownAttributeState::InvalidPropOnTag {
                name,
                tag_name,
                allowed_tags,
            } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "Property '" {name} "' is not valid on a <" {tag_name.text()} "> element."
                    },
                )
                .note(markup! {
                    "This attribute is restricted and cannot be used on this HTML element"
                })
                .note(markup! {
                       "This attribute is only allowed on: "{allowed_tags.join(",")}
                }),
            ),
        }
    }
}
