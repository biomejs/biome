<script>
/* should not generate diagnostics */
export default {
  props: {
    a: {
      type: Number,
      required: true,
    },
    b: {
      type: Number,
      default: 0,
    },
    c: {
      type: Number,
      required: false,
      default: 0,
    },
    d: {
      type: String,
      'default': 'lorem',
    },
    e: {
      type: Boolean,
    },
    f: {
      type: Boolean,
      required: false,
    },
    g: {
      type: [Boolean],
    },
    h: Boolean,
    i: [Boolean],
    j: {
      type: Object,
      default() {
        return {};
      },
    },
    k: {
      ...shared,
      default: 0,
    },
    l: {
      type: Number,
      required: isRequired,
    },
    m: {
      ...shared,
    },
    n: {
      type: Number,
      ...shared,
    },
    shorthand,
    fromVariable: options,
    fromMember: options.size,
    fromCall: makeProp(),
    custom: Person,
    mixed: [Number, Person],
  },
};
</script>
