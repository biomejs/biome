<!-- should generate diagnostics -->
<script setup>
import { useQuery } from "@tanstack/vue-query";

const id = 1;
const userId = 1;
const orgId = 2;

useQuery({
	queryKey: ["entity"],
	queryFn: () => fetchEntity(id),
});

useQuery({
	queryKey: ["users"],
	queryFn: () => fetchUser(userId, orgId),
});
</script>
