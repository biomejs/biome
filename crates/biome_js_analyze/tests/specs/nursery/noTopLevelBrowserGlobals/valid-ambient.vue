<!-- should not generate diagnostics -->
<script setup lang="ts">
declare class ClientElement extends HTMLElement {}
export declare class ExportedElement extends HTMLElement {}
declare namespace Client {
    class Element extends HTMLElement {}
}
</script>
