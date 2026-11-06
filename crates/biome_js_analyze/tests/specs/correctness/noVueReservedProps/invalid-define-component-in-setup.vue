<script setup>
import { defineComponent } from 'vue';

defineProps(['title']);

const Child = defineComponent({
    props: ['ref'],
});
</script>
