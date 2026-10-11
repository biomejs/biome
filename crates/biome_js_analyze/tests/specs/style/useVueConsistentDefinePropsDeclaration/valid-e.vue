<!-- should not generate diagnostics -->
<script setup lang="ts">
a.b.c(0);
a.b(0);
a.defineProps(0);
defineProps.a(0);
foo()(0);
</script>
