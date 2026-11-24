---
"@biomejs/biome": patch
---

Added the nursery rule [`noCanvasToImage`](https://biomejs.dev/linter/rules/no-canvas-to-image/), which reports canvases that are converted to an image or copied pixel by pixel only to be drawn onto another canvas. Pass the canvas to `drawImage()` instead.

```js
const image = await loadImage(canvas.toDataURL());
context.drawImage(image, 0, 0);
```
