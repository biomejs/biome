// Yes TS I am a module.
export type {};

const cards: NodeListOf<HTMLDivElement> = document.querySelectorAll(".card");

for (const card of cards) {
	card.addEventListener("mousemove", (e: MouseEvent) => {
		const x = e.pageX - card.offsetLeft;
		const y = e.pageY - card.offsetTop;
		card.style.setProperty("--x", `${x}px`);
		card.style.setProperty("--y", `${y}px`);
	});
}
