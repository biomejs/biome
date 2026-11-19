<!-- should not generate diagnostics -->
<script setup lang="ts">
type Name = string;
const label = { text: "Foo" };

defineOptions({ name: ("Foo" as Name) as typeof label.text });
</script>
