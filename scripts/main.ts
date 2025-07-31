import initWasm, { add } from "../build/wasm";
import PaintWorker from "./paint.worker?worker";

await initWasm();

const outputCanvas = document.getElementById("output-canvas");
const painter = new PaintWorker();

console.log(add(1, 4));
