<script setup lang="ts" generic="T   extends   Type1&Type2,U">
const a = 1;
</script>

<script setup lang="ts" generic="T extends 'a'|'b'">
const a = 1;
</script>

<script setup lang="ts" generic="T, U,">
const a = 1;
</script>

<script setup lang="ts" generic="T extends | 'loooooooooooooooooooooooooooooooooong' | 'looooooooooooooooooooooooooooooooooong', U extends LooooooooooooooooooooooooooooooooongType<AnotherLoooooooooooooooongType<NonNullable<Record<string, (Type1 & Type2 & (LoooooooooooooooooooooongType3 | LoooooooooooooooooooooongType4)) | null>[string]>>>, C">
const a = 1;
</script>

<script setup lang="ts" generic="
// comment 1:
T
extends string &
'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooong',
/**
 * comment 2
 */
U extends number, // comment 3
/** comment 4 */ C extends MyType
">
const a = 1;
</script>

<script setup generic="T   extends   U">
const a = 1;
</script>

<template>
	<not-script setup lang="ts" generic="T   extends   U"></not-script>
</template>
