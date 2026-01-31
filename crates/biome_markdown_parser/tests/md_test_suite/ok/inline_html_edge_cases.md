# Inline HTML Edge Cases

## Basic Open Tags
Simple <span>tag</span> here.
With attrs <div class="test">content</div> end.

## Self-Closing Tags
Line break: <br/> here.
With space: <br /> there.
Input: <input type="text" /> field.

## Closing Tags
Open <b>bold</b> text.
Nested <span><strong>double</strong></span> tags.

## Comments
Simple <!-- comment --> inline.
Empty <!-- --> comment.
With dashes <!--foo-bar--> here.
Leading dash <!--- --> allowed.

## Processing Instructions
XML: <?xml version="1.0"?> present.
PHP: <?php echo "test"; ?> code.

## CDATA Sections
Data: <![CDATA[some text]]> here.
Special: <![CDATA[<>&"]]> chars.

## Declarations
Standard: <!DOCTYPE html> declaration.
Lowercase: <!doctype html> declaration.
Extended: <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN"> test.

## Attributes with Quotes
Single: <div class='quoted'>text</div> end.
Double: <div class="quoted">text</div> end.
Both: <div class="outer" id='inner'>text</div> end.

## Attributes with Special Chars
Spaces: <div data-value="with spaces">text</div> end.
Multiple: <div class="a" id="b" data-x="c">text</div> end.
Unquoted: <div data-x=foo data-y=bar-baz>text</div> end.
Underscore/colon: <div _x=1 x:y=2>text</div> end.
Boolean: <div disabled>text</div> end.

## Newline Cases (should parse as inline HTML)
Allowed: <div
class="test">ok</div> tag.
Allowed: <div class="a"
>ok</div> tag.

## Priority - Autolinks Should Win
URL: <https://example.com> link.
Email: <user@example.com> address.

## Tag Names with Hyphens
Custom: <my-component>content</my-component> element.
Multiple: <my-custom-element>test</my-custom-element> tag.

## Empty Tags
Empty open: <div></div> tags.
Self close: <br/> break.
