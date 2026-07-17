<!-- 1 -->
{#if porridge.temperature > 100}
	<!-- 2 -->
	<p>too hot!</p>
	<!-- 3 -->
{:else if 80 > porridge.temperature}
	<!-- 4 -->
	<p>too cold!</p>
	<!-- 5 -->
{:else if 100 > porridge.temperature}
	<!-- 6 -->
	<p>too too cold!</p>
	<!-- 7 -->
{:else}
	<!-- 8 -->
	<p>just right!</p>
	<!-- 9 -->
{/if}
<!-- 10 -->
