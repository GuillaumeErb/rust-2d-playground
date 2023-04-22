import * as React from "react";
import { Universe } from "rust-libs/rust_libs";
import { memory } from "rust-libs/rust_libs_bg.wasm";
import "./style.css"

const CELL_SIZE = 2; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";


export const Canvas = () => {

    const canvasRef = React.useRef<HTMLCanvasElement | null>(null);
    const [universe, setUniverse] = React.useState<Universe>();
    const width = 20;
    const height = 20;

    React.useEffect(() => {
        const canvas = canvasRef.current
        if (canvas) {
            canvas.width = width;
            canvas.height = height;
            const universe = Universe.new(width, height);
            setUniverse(universe);

            const context = canvas.getContext('2d')
            if (context && universe) {
                context.imageSmoothingEnabled = false;
                drawCells(universe, context);
            }
        }
    }, [canvasRef.current === null])

    return (
        <canvas ref={canvasRef} className="canvas-playground"></canvas>
    );
}

const getIndex = (row: number, column: number, width: number) => {
    return row * width + column;
};

const drawCells = (universe: Universe, ctx: CanvasRenderingContext2D) => {
    const cellsPtr = universe.pixels();
    const height = universe.height();
    const width = universe.width();
    const pixels = new Uint8ClampedArray(memory.buffer, cellsPtr, width * height * 4);

    const myImageData = new ImageData(pixels, width, height);
    ctx.putImageData(myImageData, 0, 0);
};