import PaintWorker from "./paint.worker?worker";

const outputCanvas = document.getElementById("output-canvas");
if (!(outputCanvas instanceof HTMLCanvasElement)) {
  throw new Error("Output canvas must be a HTMLCanvasElement");
}

const offscreenCanvas = outputCanvas.transferControlToOffscreen();

const painter = new PaintWorker();
painter.postMessage(offscreenCanvas, [offscreenCanvas]);
