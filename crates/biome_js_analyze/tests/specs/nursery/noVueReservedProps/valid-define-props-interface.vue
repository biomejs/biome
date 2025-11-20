<script setup lang="ts">
// These are Vue v.2 reserved props, but they are not reserved in Vue v.3.
interface Props {
    is: string;
    slot: string;
    "slot-scope": string;
}
defineProps<Props>();
</script>
