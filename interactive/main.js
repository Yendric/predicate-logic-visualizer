import initWasm, { generate_svg_from_string } from "./pkg";

await initWasm();

document.getElementById("formula").addEventListener("input", (e) => {
    try {
        document.getElementById("out").innerHTML = generate_svg_from_string(e.target.value);
    } catch {
        document.getElementById("out").innerText = `Error: `;
    }
});
