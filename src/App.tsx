import * as React from "react";
import * as wasm from 'rust-libs';
// I have to do that for wasm to be bundled and
// for wasm to be available at runtime despite
// webpack wasm experimental. To be investigated.
require("rust-libs");

export const App: React.FC<{}> = () => {
    React.useEffect(() => { console.log("toto"); }, []);
    return (
        <>
            <button onClick={() => { wasm.greet(); }}>Toto</button>
            <h1>Hello from React!</h1>
        </>
    );
}
