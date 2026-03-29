This paragraph has [an inline link](https://example.com) and [another link](https://example.org "Example Org") with a title.

Here is [a reference link][example] and [another one][docs] and [a collapsed ref][] and [a shortcut ref].

Multiple links in one line: [alpha](https://a.com) [beta](https://b.com) [gamma](https://c.com) [delta](https://d.com)

Images inline: ![Logo](https://example.com/logo.png) and ![Photo](photo.jpg "A nice photo").

Images by reference: ![Icon][icon-ref] and ![Banner][banner-ref] and ![Badge][].

Nested constructs: [![Image Link](img.png)](https://example.com) is an image inside a link.

And *[emphasized link](https://example.com)* and **[strong link](https://example.org)**.

Link inside emphasis: *click [here](url) for more* and **visit [this site](url) today**.

Code span with link text: `[not a link](url)` should remain as code.

[Link with **strong** text](https://example.com) and [link with *emphasis*](https://example.org).

[Link with `code` inside](https://example.com/code) the link text.

![Image with **bold** alt text](image.png) describes the image.

Complex inline: *[link **with bold**](url)* inside emphasis and **![image *with italic*](img.png)** inside strong.

[Link spanning
two lines](https://example.com) is valid CommonMark.

[URL with parens](https://example.com/path_(with)_parens) and [URL with angle brackets](<https://example.com/path with spaces>).

![Large image](https://cdn.example.com/images/hero-banner-2024-redesign.png "The new hero banner for the 2024 site redesign")

Autolinks: <https://example.com> and <user@example.com> and <http://example.org/path?q=1&r=2>.

Dense link paragraph: [A](1) [B](2) [C](3) [D](4) [E](5) [F](6) [G](7) [H](8) [I](9) [J](10) [K](11) [L](12) [M](13) [N](14) [O](15).

Reference links in sequence: [ref1][r1] [ref2][r2] [ref3][r3] [ref4][r4] [ref5][r5] [ref6][r6] [ref7][r7] [ref8][r8].

Collapsed references: [alpha][] [beta][] [gamma][] [delta][] [epsilon][] [zeta][] [eta][] [theta][].

Shortcut references: [apple] [banana] [cherry] [date] [elderberry] [fig] [grape] [honeydew].

![img1](a.png) ![img2](b.png) ![img3](c.png) ![img4](d.png) ![img5](e.png) ![img6](f.png)

Images with references: ![icon1][i1] ![icon2][i2] ![icon3][i3] ![icon4][i4] ![icon5][i5]

[Link with very long URL](https://subdomain.example.com/very/long/path/to/resource?param1=value1&param2=value2&param3=value3#fragment-identifier)

Tricky cases: [foo [bar]](url) and [foo](url "title \"escaped\"") and [](empty-text).

[Full ref link][full-ref] and [Full ref with spaces][ref with spaces].

Links adjacent to text: text[link](url)text and text![image](url)text.

***[Bold italic link](https://example.com)*** and ___[underscore bold italic link](https://example.org)___

Inline link with no title: [click me](https://example.com)
Inline link with single-quoted title: [click me](https://example.com 'Title Here')
Inline link with double-quoted title: [click me](https://example.com "Title Here")
Inline link with paren title: [click me](https://example.com (Title Here))

Nested images in links: [![alt1](img1.png) and ![alt2](img2.png)](https://example.com)

Stress test line: [a](u)[b](u)[c](u)[d](u)[e](u)[f](u)[g](u)[h](u)[i](u)[j](u)[k](u)[l](u)

![Image alt with [brackets] inside](image.png) edge case.

[Link text with ![image](icon.png) inside](https://example.com) nested image in link text.

Links near each other: [first](a) [second](b)
[third](c)
[fourth](d)

This paragraph uses [many][r1] different [reference][r2] style [links][r3] throughout
the [text][r4] to [stress][r5] test [reference][r6] link [resolution][r7] and
[matching][r8] in the [parser][r9].

Final paragraph with mixed [inline](https://example.com) and [reference][final-ref] links
plus ![inline image](last.png) and ![reference image][last-img] to wrap things up.

[example]: https://example.com "Example Site"
[docs]: https://docs.example.com
[a collapsed ref]: https://collapsed.example.com
[a shortcut ref]: https://shortcut.example.com
[icon-ref]: https://example.com/icon.svg
[banner-ref]: https://example.com/banner.jpg
[Badge]: https://img.shields.io/badge/test-passing-green
[r1]: https://ref1.example.com
[r2]: https://ref2.example.com
[r3]: https://ref3.example.com
[r4]: https://ref4.example.com
[r5]: https://ref5.example.com
[r6]: https://ref6.example.com
[r7]: https://ref7.example.com
[r8]: https://ref8.example.com
[r9]: https://ref9.example.com
[alpha]: https://alpha.example.com
[beta]: https://beta.example.com
[gamma]: https://gamma.example.com
[delta]: https://delta.example.com
[epsilon]: https://epsilon.example.com
[zeta]: https://zeta.example.com
[eta]: https://eta.example.com
[theta]: https://theta.example.com
[apple]: https://apple.example.com
[banana]: https://banana.example.com
[cherry]: https://cherry.example.com
[date]: https://date.example.com
[elderberry]: https://elderberry.example.com
[fig]: https://fig.example.com
[grape]: https://grape.example.com
[honeydew]: https://honeydew.example.com
[i1]: icon1.svg
[i2]: icon2.svg
[i3]: icon3.svg
[i4]: icon4.svg
[i5]: icon5.svg
[full-ref]: https://full.example.com "Full Reference"
[ref with spaces]: https://spaces.example.com
[final-ref]: https://final.example.com
[last-img]: last-ref.png "Last Image"
