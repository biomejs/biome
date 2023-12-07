var cards = document.querySelectorAll(".card");

cards.forEach((card: any) => {
    card.addEventListener("mousemove", (e: MouseEvent) => {
        var x = e.pageX - card.offsetLeft;
        var y = e.pageY - card.offsetTop;
        card.style.setProperty("--x", x + "px");
        card.style.setProperty("--y", y + "px");
    });
});