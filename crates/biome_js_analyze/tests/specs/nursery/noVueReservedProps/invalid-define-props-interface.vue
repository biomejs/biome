<script setup lang="ts">
; // this is a hack because vue files are still parsed as js/ts files.
interface Props {
    ref: string,
    key: string,
    foo: string,
}
defineProps<Props>();
</script>
