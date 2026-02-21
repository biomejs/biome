These should NOT be parsed as link reference definitions.
They should fall back to paragraph parsing.

Unterminated angle bracket destination:
[unterminated-angle]: <http://example.com

Trailing text after destination:
[trailing-text]: /url trailing garbage

Trailing text after title:
[trailing-after-title]: /url "title" extra

Empty label is invalid:
[]: /url
