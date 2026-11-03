<script>
/* should generate diagnostics */
export default {
  props: {
    greeting_text: String,
    'greeting-text': String,
    'greeting_text2': String,
    ['computed-name']: String,
    [('parenthesized-name')]: String,
    [`template-name`]: String,
    shorthand_prop,
    'abc-123-def': String,
    _item: String,
    GreetingText: String,
    $actionEl: String,
    '漢字Label': String,
  }
}
</script>
