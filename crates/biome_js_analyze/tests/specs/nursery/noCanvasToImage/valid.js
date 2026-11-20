/* should not generate diagnostics */
context.drawImage(canvas, 0, 0);
context.drawImage(image, 0, 0);
context.drawImage(await image, 0, 0);
context.drawImage(await loadImage(url), 0, 0);
context.drawImage(await loadImage(...canvas.toDataURL()), 0, 0);
context.drawImage(await loadImage(canvas.toBlob()), 0, 0);
context.drawImage(await loadImage(canvas["toDataURL"]()), 0, 0);
context.drawImage(await loadImage(canvas.toDataURL?.()), 0, 0);
context.drawImage(await loadImage(canvas?.toDataURL()), 0, 0);
context.drawImage(await loadImage?.(canvas.toDataURL()), 0, 0);
context.drawImage?.(await loadImage(canvas.toDataURL()), 0, 0);
context?.drawImage(await loadImage(canvas.toDataURL()), 0, 0);
context.drawImage(await loadImage(chart.toDataURL()), 0, 0);
context.drawImage(await loadImage(this["canvas"].toDataURL()), 0, 0);
context.drawImage(await loadImage(canvas.toDataURL), 0, 0);
context.drawImage(await image.loadImage(canvas.toDataURL()), 0, 0);
context.drawImage(...images, 0, 0);
context.drawImage();
context["drawImage"](await loadImage(canvas.toDataURL()), 0, 0);
context.putImageData(imageData, 0, 0);
context.putImageData(await imageData, 0, 0);
context.putImageData(...context.getImageData(0, 0, width, height));
context.putImageData(context["getImageData"](0, 0, width, height), 0, 0);
context.putImageData(context.getImageData?.(0, 0, width, height), 0, 0);
context.putImageData(context?.getImageData(0, 0, width, height), 0, 0);
context.putImageData?.(context.getImageData(0, 0, width, height), 0, 0);
context?.putImageData(context.getImageData(0, 0, width, height), 0, 0);
context.drawImage(context.getImageData(0, 0, width, height), 0, 0);
context.putImageData(await loadImage(canvas.toDataURL()), 0, 0);
const url = canvas.toDataURL();

{
	const image = await loadImage(chart.toDataURL());
	context.drawImage(image, 0, 0);
}

{
	const image = await loadImage(url);
	context.drawImage(image, 0, 0);
}

{
	const image = await loadImage(canvas.toBlob());
	context.drawImage(image, 0, 0);
}

{
	let image = await loadImage(canvas.toDataURL());
	context.drawImage(image, 0, 0);
}

{
	const image = await loadImage(canvas.toDataURL());
	console.log(image);
	context.drawImage(image, 0, 0);
}

{
	const [image] = await loadImage(canvas.toDataURL());
	context.drawImage(image, 0, 0);
}

{
	function draw() {
		context.drawImage(image, 0, 0);
	}
	const image = await loadImage(canvas.toDataURL());
}

{
	const imageData = context.getImageData(0, 0, width, height);
	imageData.data[0] = 0;
	context.putImageData(imageData, 0, 0);
}

{
	let imageData = context.getImageData(0, 0, width, height);
	otherContext.putImageData(imageData, 0, 0);
}

{
	const imageData = context.getImageData(0, 0, width, height);
	console.log(imageData);
	otherContext.putImageData(imageData, 0, 0);
}

{
	function draw() {
		otherContext.putImageData(imageData, 0, 0);
	}
	const imageData = context.getImageData(0, 0, width, height);
}
