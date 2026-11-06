<!-- should not generate diagnostics -->
<script setup>
import { useCounter } from "./useCounter";

const { count, increment } = useCounter();
</script>
