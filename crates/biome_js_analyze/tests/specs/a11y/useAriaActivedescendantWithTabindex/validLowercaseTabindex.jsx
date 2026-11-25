/* should not generate diagnostics */
<>
	<CustomComponent />;
	<CustomComponent aria-activedescendant={someID} />;
	<CustomComponent aria-activedescendant={someID} tabindex={0} />;
	<CustomComponent aria-activedescendant={someID} tabindex={-1} />;
	<div />;
	<input />;
	<div tabindex={0} />;
	<div aria-activedescendant={someID} tabindex={0} />;
	<div aria-activedescendant={someID} tabindex="0" />;
	<div aria-activedescendant={someID} tabindex={1} />;
	<div aria-activedescendant={someID} tabindex={-1} />;
	<div aria-activedescendant={someID} tabindex="-1" />;
	<input aria-activedescendant={someID} />;
	<input aria-activedescendant={someID} tabindex={0} />;
	<input aria-activedescendant={someID} tabindex={-1} />;
</>;
