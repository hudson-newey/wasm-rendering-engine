import initWasm, {
  camera_backwards,
  camera_down,
  camera_forwards,
  camera_left,
  camera_right,
  camera_up,
  generate_frame,
} from "../build/wasm";

export const CAMERA_FORWARDS = "forwards";
export const CAMERA_BACKWARDS = "backwards";
export const CAMERA_LEFT = "left";
export const CAMERA_RIGHT = "right";
export const CAMERA_DOWN = "down";
export const CAMERA_UP = "up";

class CubePainter {
  private ctx: OffscreenCanvasRenderingContext2D;
  private readonly width: number;
  private readonly height: number;

  public constructor(private readonly canvas: OffscreenCanvas) {
    const context = this.canvas.getContext("2d");
    if (!context) {
      throw new Error("Failed to initialize offscreen rendering context");
    }

    this.ctx = context;

    this.width = this.canvas.width;
    this.height = this.canvas.height;
  }

  public async start() {
    this.paintNext();
  }

  private paintNext() {
    const newData = generate_frame(this.width, this.height);

    const imageData = new ImageData(
      new Uint8ClampedArray(newData),
      this.width,
      this.height
    );
    this.ctx.putImageData(imageData, 0, 0);

    // Only request the next frame after the current frame has completed.
    // This means that if the computer is slow, the animation will prefer
    // to run slowly rather than drop frames.
    requestAnimationFrame(() => this.paintNext());
  }
}

let doneInit = false;
onmessage = async (event: any) => {
  if (typeof event.data === "string") {
    switch (event.data) {
      case CAMERA_FORWARDS: {
        camera_forwards();
        break;
      }

      case CAMERA_BACKWARDS: {
        camera_backwards();
        break;
      }

      case CAMERA_LEFT: {
        camera_left();
        break;
      }

      case CAMERA_RIGHT: {
        camera_right();
        break;
      }

      case CAMERA_UP: {
        camera_up();
        break;
      }

      case CAMERA_DOWN: {
        camera_down();
        break;
      }
    }
    return;
  }

  console.assert(!doneInit, "Attempted to initialize multiple times");
  await initWasm();

  const data = event.data;

  const cubePainter = new CubePainter(data);
  cubePainter.start();
};
