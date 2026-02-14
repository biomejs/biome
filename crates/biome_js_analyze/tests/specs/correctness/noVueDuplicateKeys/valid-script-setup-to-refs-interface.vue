<!-- should not emit diagnostics -->

<script setup lang="ts">
import { toRefs } from 'vue';
interface Props {
    videoUrl: string;
    thumbnail?: string;
}
const props = defineProps<Props>()
const { videoUrl, thumbnail } = toRefs(props)
</script>
