import * as React from "react";
import { Canvas as WasmCanvas } from "rust-libs/rust_libs";
import { memory } from "rust-libs/rust_libs_bg.wasm";
import "./style.css"

export const Canvas = () => {

    const canvasRef = React.useRef<HTMLCanvasElement | null>(null);
    const [universe, setUniverse] = React.useState<WasmCanvas>();
    const width = 20;
    const height = 20;

    React.useEffect(() => {
        const canvas = canvasRef.current
        if (canvas) {
            canvas.width = width;
            canvas.height = height;
            const universe = WasmCanvas.new(width, height);
            setUniverse(universe);

            const context = canvas.getContext('2d')
            if (context && universe) {
                context.imageSmoothingEnabled = false;
                draw(universe, context);
            }
        }
    }, [canvasRef.current === null])

    console.log(universe && universe.render())

    return (
        <>
            <canvas ref={canvasRef} className="canvas-playground"></canvas>
            <br />
            <pre>
                {universe && universe.render()}
            </pre>
        </>
    );
}

const getIndex = (row: number, column: number, width: number) => {
    return row * width + column;
};

const draw = (universe: WasmCanvas, ctx: CanvasRenderingContext2D) => {
    const cellsPtr = universe.pixels();
    const height = universe.height();
    const width = universe.width();
    const pixels = new Uint8ClampedArray(memory.buffer, cellsPtr, width * height * 4);

    const myImageData = new ImageData(pixels, width, height);
    ctx.putImageData(myImageData, 0, 0);
};