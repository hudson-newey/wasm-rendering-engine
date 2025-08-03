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

document.addEventListener("keydown", (event: KeyboardEvent) => {
  switch (event.key.toLowerCase()) {
    case "w": {
      painter.postMessage(CAMERA_FORWARDS);
      break;
    }

    case "s": {
      painter.postMessage(CAMERA_BACKWARDS);
      break;
    }

    case "a": {
      painter.postMessage(CAMERA_LEFT);
      break;
    }

    case "d": {
      console.debug("right");
      painter.postMessage(CAMERA_RIGHT);
      break;
    }

    case " ": {
      event.preventDefault();
      painter.postMessage(CAMERA_UP);
      break;
    }

    case "z": {
      event.preventDefault();
      painter.postMessage(CAMERA_DOWN);
      break;
    }
  }
});
