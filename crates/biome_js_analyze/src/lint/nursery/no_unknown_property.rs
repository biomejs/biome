use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_analyze::{RuleDomain, RuleSource};
use biome_console::markup;
use biome_js_syntax::AnyJsxAttributeName;
use biome_js_syntax::{AnyJsxElementName, JsxAttribute, jsx_ext::AnyJsxElement};
use biome_rowan::AstNode;
use biome_rule_options::no_unknown_property::NoUnknownPropertyOptions;
use regex::Regex;

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
    ///   "noUnknownProperty": {
    ///     "options": {
    ///       "ignore": ["custom-attribute", "non-standard-prop"]
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// ### `requireDataLowercase`
    ///
    /// When set to `true`, requires `data-*` attributes to contain only lowercase characters.
    /// React will issue a warning when `data-*` attributes contain uppercase characters.
    ///
    /// **Default**: `false`
    ///
    /// ```json
    /// {
    ///   "noUnknownProperty": {
    ///     "options": {
    ///       "requireDataLowercase": true
    ///     }
    ///   }
    /// }
    /// ```
    ///
    pub NoUnknownProperty {
        version: "next",
        name: "noUnknownProperty",
        language: "jsx",
        domains: &[RuleDomain::React],
        sources: &[
            RuleSource::EslintReactX("no-unknown-property").same(),
            RuleSource::EslintReactXyz("no-unknown-property").same(),
        ],
        recommended: false,
    }
}

/**
 * Popover API properties added in React 19
 */
// const POPOVER_API_PROPS: &[&str] = &[
//     "popover",
//     "popoverTarget",
//     "popoverTargetAction",
//     "onToggle",
//     "onBeforeToggle",
// ];

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
    ("accept-charset", "acceptCharset"),
    ("class", "className"),
    ("crossorigin", "crossOrigin"),
    ("for", "htmlFor"),
    ("http-equiv", "httpEquiv"),
    ("nomodule", "noModule"),
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

const DOM_PROPERTY_NAMES: &[&str] = &[
    // Single word properties
    "accept",
    "action",
    "accumulate",
    "additive",
    "allow",
    "alphabetic",
    "alt",
    "amplitude",
    "as",
    "ascent",
    "async",
    "azimuth",
    "bbox",
    "begin",
    "bias",
    "buffered",
    "by",
    "capture",
    "challenge",
    "children",
    "cite",
    "clip",
    "code",
    "cols",
    "content",
    "controls",
    "coords",
    "csp",
    "cursor",
    "cx",
    "cy",
    "d",
    "data",
    "decelerate",
    "decoding",
    "default",
    "defer",
    "descent",
    "dir",
    "direction",
    "disabled",
    "display",
    "divisor",
    "draggable",
    "dur",
    "dx",
    "dy",
    "elevation",
    "end",
    "exponent",
    "fill",
    "filter",
    "form",
    "format",
    "fr",
    "from",
    "fx",
    "fy",
    "g1",
    "g2",
    "hanging",
    "headers",
    "height",
    "hidden",
    "high",
    "href",
    "hreflang",
    "icon",
    "id",
    "ideographic",
    "importance",
    "in",
    "in2",
    "inert",
    "integrity",
    "intercept",
    "k",
    "k1",
    "k2",
    "k3",
    "k4",
    "kerning",
    "key",
    "kind",
    "label",
    "lang",
    "language",
    "list",
    "loading",
    "local",
    "loop",
    "low",
    "manifest",
    "mask",
    "max",
    "media",
    "method",
    "min",
    "mode",
    "multiple",
    "muted",
    "name",
    "nonce",
    "offset",
    "open",
    "operator",
    "optimum",
    "order",
    "orient",
    "orientation",
    "origin",
    "overflow",
    "part",
    "path",
    "pattern",
    "ping",
    "placeholder",
    "points",
    "poster",
    "preload",
    "profile",
    "property",
    "r",
    "radius",
    "ref",
    "rel",
    "required",
    "results",
    "restart",
    "reversed",
    "role",
    "rows",
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
    "size",
    "sizes",
    "slot",
    "slope",
    "span",
    "spacing",
    "speed",
    "src",
    "start",
    "stemh",
    "stemv",
    "step",
    "string",
    "stroke",
    "style",
    "summary",
    "target",
    "title",
    "to",
    "transform",
    "translate",
    "type",
    "u1",
    "u2",
    "unicode",
    "value",
    "values",
    "version",
    "visibility",
    "width",
    "widths",
    "wmode",
    "wrap",
    "x",
    "x1",
    "x2",
    "xmlns",
    "y",
    "y1",
    "y2",
    "z",
    // Two word properties
    "acceptCharset",
    "accessKey",
    "accentHeight",
    "alignmentBaseline",
    "arabicForm",
    "attributeName",
    "attributeType",
    "autoCapitalize",
    "autoComplete",
    "autoCorrect",
    "autoFocus",
    "autoPictureInPicture",
    "autoPlay",
    "autoSave",
    "baseFrequency",
    "baseProfile",
    "baselineShift",
    "border",
    "calcMode",
    "capHeight",
    "cellPadding",
    "cellSpacing",
    "classID",
    "className",
    "clipPath",
    "clipPathUnits",
    "clipRule",
    "codeBase",
    "colSpan",
    "colorInterpolation",
    "colorInterpolationFilters",
    "colorProfile",
    "colorRendering",
    "contentEditable",
    "contentScriptType",
    "contentStyleType",
    "contextMenu",
    "controlsList",
    "crossOrigin",
    "dangerouslySetInnerHTML",
    "dateTime",
    "defaultChecked",
    "defaultValue",
    "diffuseConstant",
    "disablePictureInPicture",
    "disableRemotePlayback",
    "dominantBaseline",
    "edgeMode",
    "enableBackground",
    "encType",
    "enterKeyHint",
    "exportParts",
    "fetchPriority",
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
    "formAction",
    "formEncType",
    "formMethod",
    "formNoValidate",
    "formTarget",
    "frameBorder",
    "glyphName",
    "glyphOrientationHorizontal",
    "glyphOrientationVertical",
    "glyphRef",
    "gradientTransform",
    "gradientUnits",
    "horizAdvX",
    "horizOriginX",
    "hrefLang",
    "htmlFor",
    "httpEquiv",
    "imageRendering",
    "imageSizes",
    "imageSrcSet",
    "inputMode",
    "isMap",
    "itemID",
    "itemProp",
    "itemRef",
    "itemScope",
    "itemType",
    "kernelMatrix",
    "kernelUnitLength",
    "keyParams",
    "keyPoints",
    "keySplines",
    "keyTimes",
    "keyType",
    "lengthAdjust",
    "letterSpacing",
    "lightingColor",
    "limitingConeAngle",
    "marginHeight",
    "marginWidth",
    "markerEnd",
    "markerHeight",
    "markerMid",
    "markerStart",
    "markerUnits",
    "markerWidth",
    "maskContentUnits",
    "maskUnits",
    "mathematical",
    "maxLength",
    "mediaGroup",
    "minLength",
    "noValidate",
    "numOctaves",
    "onAbort",
    "onAbortCapture",
    "onAnimationEnd",
    "onAnimationEndCapture",
    "onAnimationIteration",
    "onAnimationStart",
    "onAnimationStartCapture",
    "onAuxClick",
    "onAuxClickCapture",
    "onBeforeInput",
    "onBeforeInputCapture",
    "onbeforetoggle",
    "onBlur",
    "onBlurCapture",
    "onCanPlay",
    "onCanPlayCapture",
    "onCanPlayThrough",
    "onCanPlayThroughCapture",
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
    "onLostPointerCaptureCapture",
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
    "ontoggle",
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
    "overlinePosition",
    "overlineThickness",
    "paintOrder",
    "panose1",
    "pathLength",
    "patternContentUnits",
    "patternTransform",
    "patternUnits",
    "pointerEvents",
    "pointsAtX",
    "pointsAtY",
    "pointsAtZ",
    "popover",
    "popovertarget",
    "popovertargetaction",
    "preserveAlpha",
    "preserveAspectRatio",
    "primitiveUnits",
    "radioGroup",
    "readOnly",
    "referrerPolicy",
    "refX",
    "refY",
    "rendering-intent",
    "repeatCount",
    "repeatDur",
    "requiredExtensions",
    "requiredFeatures",
    "rowSpan",
    "shapeRendering",
    "specularConstant",
    "specularExponent",
    "spellCheck",
    "spreadMethod",
    "srcDoc",
    "srcLang",
    "srcSet",
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
    "suppressContentEditableWarning",
    "suppressHydrationWarning",
    "surfaceScale",
    "systemLanguage",
    "tabIndex",
    "tableValues",
    "targetX",
    "targetY",
    "textAnchor",
    "textDecoration",
    "textLength",
    "textRendering",
    "transformOrigin",
    "underlinePosition",
    "underlineThickness",
    "unicodeBidi",
    "unicodeRange",
    "unitsPerEm",
    "useMap",
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
    "xmlnsXlink",
    "xmlSpace",
    "yChannelSelector",
    "zoomAndPan",
];

fn normalize_attribute_case(name: &str) -> &str {
    DOM_PROPERTIES_IGNORE_CASE
        .iter()
        .find(|element| element.eq_ignore_ascii_case(name))
        .unwrap_or(&name)
}

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
    ARIA_PROPERTIES.iter().any(|&element| element == name)
}

fn is_valid_html_tag_in_jsx(node: &AnyJsxElement, tag_name: &str) -> bool {
    let tag_convention = Regex::new(r"^[a-z][^-]*$").unwrap();

    if tag_convention.is_match(&tag_name) {
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

fn has_uppercase(name: &str) -> bool {
    name.contains(char::is_uppercase)
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

    DOM_PROPERTY_NAMES
        .iter()
        .find(|&&element| element.eq_ignore_ascii_case(name))
        .copied()
}

impl Rule for NoUnknownProperty {
    type Query = Manifest<JsxAttribute>;
    type State = NoUnknownPropertyDiagnostic;
    type Signals = Option<Self::State>;
    type Options = NoUnknownPropertyOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let options = ctx.options();

        let node_name = match node.name().ok()? {
            AnyJsxAttributeName::JsxName(name) => {
                name.value_token().ok()?.text_trimmed().to_string()
            }
            AnyJsxAttributeName::JsxNamespaceName(name) => {
                let namespace = name.namespace().ok()?.value_token().ok()?;
                let name = &name.name().ok()?.value_token().ok()?;
                // There could be better way, but i couldn't extract namespaced attributes
                // For e.g xlink:href
                // without manually concatenating with ':'
                namespace.text_trimmed().to_string() + ":" + name.text_trimmed()
            }
        };

        if options.ignore.contains(&node_name) {
            return None;
        }

        let name = normalize_attribute_case(&node_name);
        let parent = node.syntax().parent()?.parent()?;
        let element = AnyJsxElement::cast_ref(&parent)?;

        // Ignore tags like <Foo.bar />
        if tag_name_has_dot(&element)? {
            return None;
        }

        // Handle data-* attributes
        if is_valid_data_attribute(name) {
            if options.require_data_lowercase && has_uppercase(&name) {
                return Some(NoUnknownPropertyDiagnostic::DataLowercaseRequired {
                    name: name.to_string(),
                    lowercase_name: name.to_lowercase(),
                });
            }
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

        let allowed_tags = get_allowed_tags(&name);

        if let Some(allowed_tags) = allowed_tags {
            if !allowed_tags.contains(&tag_name.trim()) {
                return Some(NoUnknownPropertyDiagnostic::InvalidPropOnTag {
                    name: name.to_string(),
                    tag_name: tag_name.to_string(),
                    allowed_tags: allowed_tags.join(","),
                });
            }
            return None;
        }

        if let Some(standard_name) = get_standard_name(name) {
            if standard_name != name {
                return Some(NoUnknownPropertyDiagnostic::UnknownPropWithStandardName {
                    name: name.to_string(),
                    standard_name: standard_name.to_string(),
                });
            }
            return None;
        }

        Some(NoUnknownPropertyDiagnostic::UnknownProp {
            name: name.to_string(),
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        match state {
            NoUnknownPropertyDiagnostic::UnknownProp { name } => Some(
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
            NoUnknownPropertyDiagnostic::UnknownPropWithStandardName {
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
            NoUnknownPropertyDiagnostic::InvalidPropOnTag {
                name,
                tag_name,
                allowed_tags,
            } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "Property '" {name} "' is not valid on a <" {tag_name} "> element."
                    },
                )
                .note(markup! {
                    "This attribute is restricted and cannot be used on this HTML element"
                })
                .note(markup! {
                       "This attribute is only allowed on: "{allowed_tags}
                }),
            ),
            NoUnknownPropertyDiagnostic::DataLowercaseRequired {
                name,
                lowercase_name,
            } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "data-* attribute '"{name}"' should use lowercase naming."
                    },
                )
                .note(markup! {
                    "HTML data-* attributes must use lowercase letters to be valid."
                })
                .note(markup! {
                    "Change '"{name}"' to '"{lowercase_name}"' to follow HTML standards."
                }),
            ),
        }
    }
}
