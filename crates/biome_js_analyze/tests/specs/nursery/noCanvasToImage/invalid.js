/* should generate diagnostics */
context.drawImage(loadImage(canvas.toDataURL()), 0, 0);
context.drawImage(await loadImage(canvas.toDataURL()), 0, 0);
context.drawImage(await loadImage(sourceCanvas.toDataURL()), 0, 0);
context.drawImage(await loadImage(this.canvas.toDataURL()), 0, 0);
context.drawImage(await loadImage(CANVAS.toDataURL("image/png")), 0, 0);
context.drawImage((await loadImage((canvas).toDataURL())), 0, 0);
(context.drawImage)(await loadImage(canvas.toDataURL()), 0, 0);
context.drawImage(await loadImage((canvas.toDataURL())), 0, 0);

{
	const image = await loadImage(canvas.toDataURL());
	context.drawImage(image, 0, 0);
}

{
	const image = loadImage(canvas.toDataURL("image/jpeg"));
	context.drawImage(await image, 0, 0);
}

context.putImageData(context.getImageData(0, 0, width, height), 0, 0);
context2.putImageData(context1.getImageData(0, 0, width, height), 0, 0);

{
	const imageData = context1.getImageData(0, 0, width, height);
	context2.putImageData(imageData, 0, 0);
}

function draw() {
	const imageData = context1.getImageData(0, 0, width, height);
	return context2.putImageData(imageData, 0, 0);
}
