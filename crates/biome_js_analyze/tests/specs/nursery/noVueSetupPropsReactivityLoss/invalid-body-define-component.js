// Invalid: destructuring in defineComponent
import { defineComponent } from 'vue';
export default defineComponent({
  setup(props) {
    const { count, msg } = props;
    return { count, msg };
  }
})
