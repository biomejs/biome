# Invalid Inline HTML Cases

These should all be parsed as text, NOT as inline HTML.

## Period in Tag Name
The URL <example.com> should remain text.
Domain <test.example.com> should remain text.

## Unclosed Tags
Open bracket < followed by text.
Partial tag <div should be text.
Missing end <div class="test" should be text.
Missing name <div =foo> should be text.
Missing value <div data-x=> should be text.
Backtick in unquoted <div data-x=`a`> should be text.
Invalid name <div 1a=foo> should be text.

## Invalid Comments
Invalid start <!--> should be text.
Double dash <!-- foo -- bar --> should be text maybe.
Starts with arrow <!---> should be text.
