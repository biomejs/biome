<!-- should generate diagnostics -->

<script>
  const staticClasses = clsx("w-4 h-4");
  const templateClasses = tw.div`overflow-hidden text-ellipsis whitespace-nowrap`;
</script>

<div class={clsx("w-4 h-4", active && "px-2 py-2")}></div>
<!-- known limitations, we can't generate diagnostics for these cases yet. requires additional plumbing -->
<div class={["overflow-hidden text-ellipsis whitespace-nowrap", selected && "hover:w-6 hover:h-6"]}></div>
<div class={{ "mr-3 ml-3": active }}></div>
