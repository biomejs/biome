<script setup lang="ts">
; // this is a hack because vue files are still parsed as js/ts files.
interface Props {
    $el: string
}
defineProps<Props>();
</script>
