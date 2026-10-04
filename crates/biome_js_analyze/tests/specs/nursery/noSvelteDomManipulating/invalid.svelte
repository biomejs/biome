<!-- should generate diagnostics -->
<script>
	let foo, bar, baz, qux, el, dynamic, self, maybe;

	const remove = () => foo.remove();
	const update = () => (bar.textContent = "Update!");
	baz.appendChild(document.createElement("span"));
	baz.insertBefore(document.createElement("span"), null);
	baz.normalize();
	baz.removeChild(baz.firstChild);
	baz.replaceChild(document.createElement("span"), baz.firstChild);
	baz.after("text");
	baz.append("text");
	baz.before("text");
	baz.insertAdjacentElement("beforebegin", document.createElement("span"));
	baz.insertAdjacentHTML("beforebegin", "<span></span>");
	baz.insertAdjacentText("beforebegin", "text");
	baz.prepend("text");
	baz.replaceChildren();
	baz.replaceWith("text");
	qux.innerHTML = "<span></span>";
	qux.outerHTML = "<span></span>";
	qux.innerText = "text";
	qux.outerText = "text";
	qux.textContent += "text";
	qux["textContent"] = "text";
	el["remove"]();
	el?.remove();
	(el).remove();
	(el.remove)();
	(el.textContent) = "text";
	dynamic.remove();
	self.remove();
	function nested() {
		maybe.remove();
	}
</script>

{#if foo}
	<div bind:this={foo}>Foo</div>
{/if}
<span bind:this={bar}></span>
<div bind:this={ baz } />
<p bind:this={qux}></p>
<input bind:this={el} />
<svelte:element this="div" bind:this={dynamic} />
<my-element bind:this={self}></my-element>
<div bind:this={maybe}></div>
<button onclick={() => foo.remove()}>Remove</button>
<button onclick={() => (bar.innerHTML = "")}>Clear</button>
