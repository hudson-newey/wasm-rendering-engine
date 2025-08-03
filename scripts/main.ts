import { Keybindings, KeyboardListener } from "./keyboard";
import {
  CAMERA_BACKWARDS,
  CAMERA_DOWN,
  CAMERA_FORWARDS,
  CAMERA_LEFT,
  CAMERA_RIGHT,
  CAMERA_UP,
} from "./paint.worker";
import PaintWorker from "./paint.worker?worker";

const outputCanvas = document.getElementById("output-canvas");
if (!(outputCanvas instanceof HTMLCanvasElement)) {
  throw new Error("Output canvas must be a HTMLCanvasElement");
}

const offscreenCanvas = outputCanvas.transferControlToOffscreen();

const painter = new PaintWorker();
painter.postMessage(offscreenCanvas, [offscreenCanvas]);

const shortcuts: Keybindings = new Map([
  ["w", () => painter.postMessage(CAMERA_FORWARDS)],
  ["s", () => painter.postMessage(CAMERA_BACKWARDS)],
  ["a", () => painter.postMessage(CAMERA_LEFT)],
  ["d", () => painter.postMessage(CAMERA_RIGHT)],
  [" ", () => painter.postMessage(CAMERA_UP)],
  ["shift", () => painter.postMessage(CAMERA_DOWN)],
]);

new KeyboardListener(shortcuts);
