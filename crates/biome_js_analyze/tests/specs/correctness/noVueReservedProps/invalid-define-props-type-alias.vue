<script setup lang="ts">
; // this is a hack because vue files are still parsed as js/ts files.
type Props = {
    ref: string,
    key: string,
    foo: string,
};
defineProps<Props>();
</script>
