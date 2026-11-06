//! Conformance tests for `MarkdownParserOptions::with_cjk_friendly_emphasis`.
//!
//! The cases come from the test suite of the CommonMark CJK-friendly
//! amendments, as collected in pulldown-cmark's
//! `specs/cjk_friendly_emphasis.txt`:
//! <https://github.com/tats-u/markdown-cjk-friendly/tree/ee93f3c2dffc8c7eb25c7a1c9f3f962279ac247d/testcases>
//!
//! The markdown-cjk-friendly test cases are licensed under the MIT License:
//! Copyright (c) 2024 Tatsunori Uchino, and authors and contributors of
//! original packages.
//!
//! Single-tilde strikethrough cases are omitted because the parser only
//! supports `~~`. The `~~!~~` case is omitted because GFM applies the flanking
//! rules to `~~`, while pulldown-cmark doesn't.

use biome_markdown_parser::{MarkdownParserOptions, document_to_html, parse_markdown};
use biome_markdown_syntax::MdRoot;
use biome_rowan::AstNode;

fn render_html(input: &str, options: MarkdownParserOptions) -> String {
    let parsed = parse_markdown(input, options);
    let document = MdRoot::cast(parsed.syntax()).expect("the parser always returns a document");
    document_to_html(
        &document,
        parsed.list_tightness(),
        parsed.list_item_indents(),
        parsed.quote_indents(),
    )
}

#[test]
fn cjk_friendly_emphasis_suite() {
    let options = MarkdownParserOptions::default()
        .with_gfm(true)
        .with_cjk_friendly_emphasis(true);

    for (input, expected) in [
        (
            "**このアスタリスクは強調記号として認識されず、そのまま表示されます。**この文のせいで。",
            "<p><strong>このアスタリスクは強調記号として認識されず、そのまま表示されます。</strong>この文のせいで。</p>",
        ),
        (
            "**该星号不会被识别，而是直接显示。**这是因为它没有被识别为强调符号。",
            "<p><strong>该星号不会被识别，而是直接显示。</strong>这是因为它没有被识别为强调符号。</p>",
        ),
        (
            "**이 별표는 강조 표시로 인식되지 않고 그대로 표시됩니다(이 괄호 때문에)**이 문장 때문에.",
            "<p><strong>이 별표는 강조 표시로 인식되지 않고 그대로 표시됩니다(이 괄호 때문에)</strong>이 문장 때문에.</p>",
        ),
        (
            "これは**私のやりたかったこと。**だからするの。",
            "<p>これは<strong>私のやりたかったこと。</strong>だからするの。</p>",
        ),
        (
            "**[製品ほげ](./product-foo)**と**[製品ふが](./product-bar)**をお試しください",
            r#"<p><strong><a href="./product-foo">製品ほげ</a></strong>と<strong><a href="./product-bar">製品ふが</a></strong>をお試しください</p>"#,
        ),
        (
            "単語と**[単語と](word-and)**単語",
            r#"<p>単語と<strong><a href="word-and">単語と</a></strong>単語</p>"#,
        ),
        (
            "**これは太字になりません。**ご注意ください。",
            "<p><strong>これは太字になりません。</strong>ご注意ください。</p>",
        ),
        (
            "カッコに注意**（太字にならない）**文が続く場合に要警戒。",
            "<p>カッコに注意<strong>（太字にならない）</strong>文が続く場合に要警戒。</p>",
        ),
        (
            "**[リンク](https://example.com)**も注意。（画像も同様）",
            r#"<p><strong><a href="https://example.com">リンク</a></strong>も注意。（画像も同様）</p>"#,
        ),
        (
            r#"先頭の**\`コード\`も注意。**"#,
            "<p>先頭の<strong>`コード`も注意。</strong></p>",
        ),
        (
            r#"**末尾の\`コード\`**も注意。"#,
            "<p><strong>末尾の`コード`</strong>も注意。</p>",
        ),
        (
            "税込**¥10,000**で入手できます。",
            "<p>税込<strong>¥10,000</strong>で入手できます。</p>",
        ),
        ("正解は**④**です。", "<p>正解は<strong>④</strong>です。</p>"),
        (
            "太郎は**「こんにちわ」**といった",
            "<p>太郎は<strong>「こんにちわ」</strong>といった</p>",
        ),
        (
            r#"太郎は**"こんにちわ"**といった"#,
            "<p>太郎は<strong>&quot;こんにちわ&quot;</strong>といった</p>",
        ),
        (
            "太郎は**こんにちわ**といった",
            "<p>太郎は<strong>こんにちわ</strong>といった</p>",
        ),
        (
            "太郎は**「Hello」**といった",
            "<p>太郎は<strong>「Hello」</strong>といった</p>",
        ),
        (
            r#"太郎は**"Hello"**といった"#,
            "<p>太郎は<strong>&quot;Hello&quot;</strong>といった</p>",
        ),
        (
            "太郎は**Hello**といった",
            "<p>太郎は<strong>Hello</strong>といった</p>",
        ),
        (
            "太郎は**「Oh my god」**といった",
            "<p>太郎は<strong>「Oh my god」</strong>といった</p>",
        ),
        (
            r#"太郎は**"Oh my god"**といった"#,
            "<p>太郎は<strong>&quot;Oh my god&quot;</strong>といった</p>",
        ),
        (
            "太郎は**Oh my god**といった",
            "<p>太郎は<strong>Oh my god</strong>といった</p>",
        ),
        (
            "**C#**や**F#**は**「.NET」**というプラットフォーム上で動作します。",
            "<p><strong>C#</strong>や<strong>F#</strong>は<strong>「.NET」</strong>というプラットフォーム上で動作します。</p>",
        ),
        (
            "IDが**001号**になります。",
            "<p>IDが<strong>001号</strong>になります。</p>",
        ),
        (
            "IDが**００１号**になります。",
            "<p>IDが<strong>００１号</strong>になります。</p>",
        ),
        (
            "Go**「初心者」**を対象とした記事です。",
            "<p>Go<strong>「初心者」</strong>を対象とした記事です。</p>",
        ),
        (
            "**[リンク](https://example.com)**も注意。",
            r#"<p><strong><a href="https://example.com">リンク</a></strong>も注意。</p>"#,
        ),
        (
            "**⻲田太郎**と申します",
            "<p><strong>⻲田太郎</strong>と申します</p>",
        ),
        (
            "・**㋐**:選択肢１つ目",
            "<p>・<strong>㋐</strong>:選択肢１つ目</p>",
        ),
        ("**真，**她", "<p><strong>真，</strong>她</p>"),
        ("**真。**她", "<p><strong>真。</strong>她</p>"),
        ("**真、**她", "<p><strong>真、</strong>她</p>"),
        ("**真；**她", "<p><strong>真；</strong>她</p>"),
        ("**真：**她", "<p><strong>真：</strong>她</p>"),
        ("**真？**她", "<p><strong>真？</strong>她</p>"),
        ("**真！**她", "<p><strong>真！</strong>她</p>"),
        ("**真“**她", "<p><strong>真“</strong>她</p>"),
        ("**真”**她", "<p><strong>真”</strong>她</p>"),
        ("**真‘**她", "<p><strong>真‘</strong>她</p>"),
        ("**真’**她", "<p><strong>真’</strong>她</p>"),
        ("**真（**她", "<p><strong>真（</strong>她</p>"),
        ("真**（她**", "<p>真<strong>（她</strong></p>"),
        ("**真）**她", "<p><strong>真）</strong>她</p>"),
        ("**真【**她", "<p><strong>真【</strong>她</p>"),
        ("真**【她**", "<p>真<strong>【她</strong></p>"),
        ("**真】**她", "<p><strong>真】</strong>她</p>"),
        ("**真《**她", "<p><strong>真《</strong>她</p>"),
        ("真**《她**", "<p>真<strong>《她</strong></p>"),
        ("**真》**她", "<p><strong>真》</strong>她</p>"),
        ("**真—**她", "<p><strong>真—</strong>她</p>"),
        ("**真～**她", "<p><strong>真～</strong>她</p>"),
        ("**真…**她", "<p><strong>真…</strong>她</p>"),
        ("**真·**她", "<p><strong>真·</strong>她</p>"),
        ("**真〃**她", "<p><strong>真〃</strong>她</p>"),
        ("**真-**她", "<p><strong>真-</strong>她</p>"),
        ("**真々**她", "<p><strong>真々</strong>她</p>"),
        ("**真**她", "<p><strong>真</strong>她</p>"),
        ("**真，** 她", "<p><strong>真，</strong> 她</p>"),
        ("**真**，她", "<p><strong>真</strong>，她</p>"),
        (
            "**真，**&ZeroWidthSpace;她",
            "<p><strong>真，</strong>\u{200B}她</p>",
        ),
        (
            "私は**⻲田太郎**と申します",
            "<p>私は<strong>⻲田太郎</strong>と申します</p>",
        ),
        (
            "選択肢**㋐**: 1つ目の選択肢",
            "<p>選択肢<strong>㋐</strong>: 1つ目の選択肢</p>",
        ),
        (
            "**さようなら︙**と太郎はいった。",
            "<p><strong>さようなら︙</strong>と太郎はいった。</p>",
        ),
        (
            ".NET**（.NET Frameworkは不可）**では、",
            "<p>.NET<strong>（.NET Frameworkは不可）</strong>では、</p>",
        ),
        (
            "「禰\u{E0100}」の偏は示ではなく**礻**です。",
            "<p>「禰\u{E0100}」の偏は示ではなく<strong>礻</strong>です。</p>",
        ),
        (
            "Git**（注：不是GitHub）**",
            "<p>Git<strong>（注：不是GitHub）</strong></p>",
        ),
        (
            "太郎は**「こんにちわ」**といった。",
            "<p>太郎は<strong>「こんにちわ」</strong>といった。</p>",
        ),
        (
            "𰻞𰻞**（ビャンビャン）**麺",
            "<p>𰻞𰻞<strong>（ビャンビャン）</strong>麺</p>",
        ),
        (
            "𰻞𰻞**(ビャンビャン)**麺",
            "<p>𰻞𰻞<strong>(ビャンビャン)</strong>麺</p>",
        ),
        (
            "ハイパーテキストコーヒーポット制御プロトコル**(HTCPCP)**",
            "<p>ハイパーテキストコーヒーポット制御プロトコル<strong>(HTCPCP)</strong></p>",
        ),
        ("﨑**(崎)**", "<p>﨑<strong>(崎)</strong></p>"),
        (
            "国際規格**[ECMA-262](https://tc39.es/ecma262/)**",
            r#"<p>国際規格<strong><a href="https://tc39.es/ecma262/">ECMA-262</a></strong></p>"#,
        ),
        ("㐧**(第の俗字)**", "<p>㐧<strong>(第の俗字)</strong></p>"),
        (
            "𠮟**(こちらが正式表記)**",
            "<p>𠮟<strong>(こちらが正式表記)</strong></p>",
        ),
        (
            "𪜈**(トモの合略仮名)**",
            "<p>𪜈<strong>(トモの合略仮名)</strong></p>",
        ),
        ("𫠉**(馬の俗字)**", "<p>𫠉<strong>(馬の俗字)</strong></p>"),
        (
            "谺𬤲**(こだま)**石神社",
            "<p>谺𬤲<strong>(こだま)</strong>石神社</p>",
        ),
        (
            "石𮧟**(いしただら)**",
            "<p>石𮧟<strong>(いしただら)</strong></p>",
        ),
        (
            "**推荐几个框架：**React、Vue等前端框架。",
            "<p><strong>推荐几个框架：</strong>React、Vue等前端框架。</p>",
        ),
        (
            "葛\u{E0100}**(こちらが正式表記)**城市",
            "<p>葛\u{E0100}<strong>(こちらが正式表記)</strong>城市</p>",
        ),
        (
            "禰\u{E0100}**(こちらが正式表記)**豆子",
            "<p>禰\u{E0100}<strong>(こちらが正式表記)</strong>豆子</p>",
        ),
        ("𱟛**(U+317DB)**", "<p>𱟛<strong>(U+317DB)</strong></p>"),
        (
            "阿寒湖アイヌシアターイコㇿ**(Akanko Ainu Theater Ikor)**",
            "<p>阿寒湖アイヌシアターイコㇿ<strong>(Akanko Ainu Theater Ikor)</strong></p>",
        ),
        (
            "あ𛀙**(か)**よろし",
            "<p>あ𛀙<strong>(か)</strong>よろし</p>",
        ),
        (
            "𮹝**(simplified form of 龘 in China)**",
            "<p>𮹝<strong>(simplified form of 龘 in China)</strong></p>",
        ),
        (
            "大塚\u{FE00}**(U+585A U+FE00)** 大塚**(U+FA10)**",
            "<p>大塚\u{FE00}<strong>(U+585A U+FE00)</strong> 大塚<strong>(U+FA10)</strong></p>",
        ),
        (
            "〽\u{FE0E}**(庵点)**は、",
            "<p>〽\u{FE0E}<strong>(庵点)</strong>は、</p>",
        ),
        (
            "**“\u{FE01}Git”\u{FE01}**Hub",
            "<p><strong>“\u{FE01}Git”\u{FE01}</strong>Hub</p>",
        ),
        (
            "**이 [링크](https://example.kr/)**만을 강조하고 싶다.",
            r#"<p><strong>이 <a href="https://example.kr/">링크</a></strong>만을 강조하고 싶다.</p>"#,
        ),
        (
            "**스크립트(script)**라고",
            "<p><strong>스크립트(script)</strong>라고</p>",
        ),
        (
            "패키지를 발행하려면 **`npm publish`**를 실행하십시오.",
            "<p>패키지를 발행하려면 <strong><code>npm publish</code></strong>를 실행하십시오.</p>",
        ),
        (
            "**안녕(hello)**하세요.",
            "<p><strong>안녕(hello)</strong>하세요.</p>",
        ),
        ("ᅡ**(a)**", "<p>ᅡ<strong>(a)</strong></p>"),
        ("**(k)**ᄏ", "<p><strong>(k)</strong>ᄏ</p>"),
        ("a**〰**a", "<p>a<strong>〰</strong>a</p>"),
        ("a**〽**a", "<p>a<strong>〽</strong>a</p>"),
        ("a**🈂**a", "<p>a<strong>🈂</strong>a</p>"),
        ("a**🈷**a", "<p>a<strong>🈷</strong>a</p>"),
        ("a**㊗**a", "<p>a<strong>㊗</strong>a</p>"),
        ("a**㊙**a", "<p>a<strong>㊙</strong>a</p>"),
        (
            "
a**a«**a",
            "<p>a**a«**a</p>",
        ),
        ("a**»a**a", "<p>a**»a**a</p>"),
        ("a**a∇**a", "<p>a**a∇**a</p>"),
        ("a**∇a**a", "<p>a**∇a**a</p>"),
        ("a**a𝜵**a", "<p>a**a𝜵**a</p>"),
        ("a**𝜵a**a", "<p>a**𝜵a**a</p>"),
        ("a**𐬻a**a", "<p>a**𐬻a**a</p>"),
        ("a**a𐬻**a", "<p>a**a𐬻**a</p>"),
        (
            "__注意__：注意事項",
            "<p><strong>注意</strong>：注意事項</p>",
        ),
        (
            "注意：__注意事項__",
            "<p>注意：<strong>注意事項</strong></p>",
        ),
        (
            "正體字。\u{FE01}_Traditional._",
            "<p>正體字。\u{FE01}<em>Traditional.</em></p>",
        ),
        (
            "正體字。\u{FE01}__Hong Kong and Taiwan.__",
            "<p>正體字。\u{FE01}<strong>Hong Kong and Taiwan.</strong></p>",
        ),
        (
            "简体字 / 新字体。\u{FE00}_Simplified._",
            "<p>简体字 / 新字体。\u{FE00}<em>Simplified.</em></p>",
        ),
        (
            "简体字 / 新字体。\u{FE00}__Mainland China or Japan.__",
            "<p>简体字 / 新字体。\u{FE00}<strong>Mainland China or Japan.</strong></p>",
        ),
        (
            "“\u{FE01}Git”\u{FE01}__Hub__",
            "<p>“\u{FE01}Git”\u{FE01}<strong>Hub</strong></p>",
        ),
        ("foo_bar_", "<p>foo_bar_</p>"),
        ("_foo_bar", "<p>_foo_bar</p>"),
        ("_foo_bar_baz_", "<p><em>foo_bar_baz</em></p>"),
        ("漢_abc_", "<p>漢_abc_</p>"),
        ("_abc_漢", "<p>_abc_漢</p>"),
        ("真~~（她~~", "<p>真<del>（她</del></p>"),
        ("~~真，~~她", "<p><del>真，</del>她</p>"),
        ("This~~is~~stricken", "<p>This<del>is</del>stricken</p>"),
        ("真_（她_", "<p>真_（她_</p>"),
        ("_真，_她", "<p>_真，_她</p>"),
        ("あ**()**あ", "<p>あ<strong>()</strong>あ</p>"),
    ] {
        similar_asserts::assert_eq!(
            format!("{expected}\n"),
            render_html(input, options.clone()),
            "input: {input:?}"
        );
    }
}

#[test]
fn cjk_friendly_emphasis_in_headings_and_tables() {
    let options = MarkdownParserOptions::default()
        .with_gfm(true)
        .with_cjk_friendly_emphasis(true);

    similar_asserts::assert_eq!(
        "<h1><strong>テスト。</strong>テスト</h1>\n",
        render_html("# **テスト。**テスト\n", options.clone())
    );
    similar_asserts::assert_eq!(
        "<table>\n<thead>\n<tr>\n<th><strong>テスト。</strong>テスト</th>\n</tr>\n</thead>\n</table>\n",
        render_html("| **テスト。**テスト |\n| --- |\n", options)
    );
}

#[test]
fn commonmark_emphasis_without_cjk_friendly_option() {
    let options = MarkdownParserOptions::default().with_gfm(true);

    for (input, expected) in [
        ("**テスト。**テスト", "<p>**テスト。**テスト</p>"),
        ("真~~（她~~", "<p>真~~（她~~</p>"),
        (
            "正體字。\u{FE01}_Traditional._",
            "<p>正體字。\u{FE01}_Traditional._</p>",
        ),
    ] {
        similar_asserts::assert_eq!(
            format!("{expected}\n"),
            render_html(input, options.clone()),
            "input: {input:?}"
        );
    }
}

#[test]
fn cjk_friendly_emphasis_changes_delimiter_pairing() {
    // Only the amendments let the middle `**` close emphasis. That also leaves
    // the last `**` without a partner, although it isn't next to CJK text.
    let input = "**テスト。**テスト a**";
    let options = MarkdownParserOptions::default().with_gfm(true);

    similar_asserts::assert_eq!(
        "<p>**テスト。<strong>テスト a</strong></p>\n",
        render_html(input, options.clone())
    );
    similar_asserts::assert_eq!(
        "<p><strong>テスト。</strong>テスト a**</p>\n",
        render_html(input, options.with_cjk_friendly_emphasis(true))
    );
}
