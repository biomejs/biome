<script setup lang="ts">
; // this is a hack because vue files are still parsed as js/ts files.
type A = {
    $el: string
};
defineProps<A>();
</script>
