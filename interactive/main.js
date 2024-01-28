import { init, WASI } from "@wasmer/wasi";
await init();

let wasi = new WASI({});
const moduleBytes = fetch("./predicate_logic_visualizer.wasm");
const module = await WebAssembly.compileStreaming(moduleBytes);
const instance = wasi.instantiate(module, {});

/* See https://stackoverflow.com/a/49020435 */
function encodeString(string) {
  /*
   0 is reserved for NULL. We're only storing one string,
   so we can always put it at the same address.
  */
  const stringStart = 4;
  const memory = instance.exports.memory;
  const encoder = new TextEncoder();
  const encodedString = encoder.encode(string);

  const asU32 = new Uint32Array(memory.buffer, stringStart, 2);
  const asBytes = new Uint8Array(
    memory.buffer,
    asU32.byteOffset + asU32.byteLength,
    encodedString.length
  );

  asBytes.set(encodedString);

  asU32[0] = asBytes.byteOffset;
  asU32[1] = asBytes.length;

  return stringStart;
}

document.getElementById("formula").addEventListener("input", (e) => {
  try {
    instance.exports.generate_svg_from_string(encodeString(e.target.value));

    document.getElementById("out").innerHTML = wasi.getStdoutString();
  } catch {
    document.getElementById("out").innerText = `Error: ${
      wasi.getStderrString().split("\n")[1]
    }`;
  }
});
