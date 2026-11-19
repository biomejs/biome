<script>
export default {
    props: {
        open: { type: Boolean, default: true },
    },
};
</script>

<script setup lang="ts">
const { visible = false } = defineProps<{ visible?: boolean }>();
</script>
