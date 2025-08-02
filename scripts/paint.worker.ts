import initWasm, { next_frame } from "../build/wasm";

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
    const canvasPixelCount = this.canvas.width * this.canvas.height;
    const canvasChannels = 4; // r, g, b, a
    const bufferSize = canvasPixelCount * canvasChannels;

    const initialBuffer = new Uint8Array(bufferSize);

    this.paintNext();
  }

  private paintNext() {
    const newData = next_frame(this.width, this.height);

    const imageData = new ImageData(
      new Uint8ClampedArray(newData),
      this.width,
      this.height
    );
    this.ctx.putImageData(imageData, 0, 0);

    // Only request the next frame after the current frame has completed.
    // This means that if the computer is slow, the animation will prefer
    // to run slowly rather than drop frames.
    // requestAnimationFrame(() => this.paintNext());
  }
}

let doneInit = false;
onmessage = async (event: any) => {
  console.assert(!doneInit, "Attempted to initialize multiple times");
  await initWasm();

  const data = event.data;

  const cubePainter = new CubePainter(data);
  cubePainter.start();
};
