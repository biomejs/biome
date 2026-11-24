<!-- should not generate diagnostics -->
<script setup>
import { useQuery } from "@tanstack/vue-query";
import { fetchTodos } from "./api";

const id = 1;
const ids = [1, 2, 3];

useQuery({ queryKey: ["entity", id], queryFn: () => fetchEntity(id) });
useQuery({ queryKey: ["todos"], queryFn: () => fetchTodos() });
useQuery({ queryKey: ["entity", id], queryFn: () => fetch("/api/entity/" + id) });
useQuery({ queryKey: ["entities", ids], queryFn: () => ids.map((id) => fetchEntity(id)) });
</script>
