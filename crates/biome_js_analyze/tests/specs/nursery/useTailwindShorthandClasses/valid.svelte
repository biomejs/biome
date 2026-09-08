<!-- should not generate diagnostics -->

<script>
	const staticClasses = clsx("size-4");
	const templateClasses = tw.div`truncate`;
</script>

<div class={clsx("size-4", active && "p-2")}></div>
<div class={["truncate", selected && "hover:size-6"]}></div>
<div class={{ "mx-3": active }}></div>

<!-- valid because this is unsupported -->
<div class={`border ${extra}`}></div>
<div class={`border-x border-y ${extra}`}></div>
