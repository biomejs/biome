<script lang="ts">
/* should generate diagnostics */
export default {
  props: {
    a: Number,
    b: [Number, String],
    c: {
      type: Number,
    },
    d: {
      type: Number,
      required: false,
    },
    e: [Boolean, String],
    f: {
      type: [Boolean, String],
    },
    g: {
      type: String,
    } as PropOptions<string>,
    h: Date,
    i: {},
    'k': String,
    ['l']: String,
    m: (Number),
  },
};
</script>
