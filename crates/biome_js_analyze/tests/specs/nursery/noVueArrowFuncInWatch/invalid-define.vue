<script>
/* should generate diagnostics */
import { defineComponent } from 'vue';

export default defineComponent({
  watch: {
    foo: (val, oldVal) => {
      console.log('new: %s, old: %s', val, oldVal)
    },
    bar: {
			handler: (val, oldVal) => {
				console.log('new: %s, old: %s', val, oldVal)
			}
    }
    short: (val, oldVal) => console.log('new: %s, old: %s', val, oldVal)
  }
})
</script>
