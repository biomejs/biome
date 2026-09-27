<script>
import { set } from 'vue'

export default {
  computed: {
    // Vue 2 exports `set` as a named function as well as hanging it off `Vue`.
    written() {
      return set(this, 'foo', 'lorem')
    },
  },
}
</script>
