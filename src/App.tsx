import * as React from "react";
import { greet } from 'rust-libs/rust_libs';
import { Canvas } from "./Canvas";
// I have to do that for wasm to be bundled and
// for wasm to be available at runtime despite
// webpack wasm experimental. To be investigated.
require("rust-libs/rust_libs");

export const App: React.FC<{}> = () => {

    const [wasm, setWasm] = React.useState<{} | undefined>(undefined);

    React.useEffect(() => {
        import("rust-libs/rust_libs").then(setWasm)
    }, []);

    if (wasm === undefined) {
        return (<></>);
    }

    return (
        <>
            <button onClick={() => { greet(); }}>Toto</button>
            <h1>Hello from React!</h1>
            <Canvas />
        </>
    );
}
