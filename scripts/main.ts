import shaderLogicResource from "../build/debug.wasm?url";
import PaintWorker from "./paint.worker?worker";

interface WasmModule {
    add: (a: number, b: number) => number;
}

const outputCanvas = document.getElementById("output-canvas");

const painter = new PaintWorker();

const { add } = await WebAssembly.instantiateStreaming(fetch(shaderLogicResource)).then(
    (obj) => obj.instance.exports as any as WasmModule,
);

console.log(add(1, 5));
