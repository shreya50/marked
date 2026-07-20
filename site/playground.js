import init, { parse_markdown } from "./pkg/marked_rs.js";

const input = document.querySelector("#markdown-input");
const gfmToggle = document.querySelector("#gfm-toggle");
const output = document.querySelector("#html-output");
const status = document.querySelector("#wasm-status");

function render() {
  output.textContent = parse_markdown(input.value, gfmToggle.checked);
}

async function start() {
  try {
    await init();
    status.textContent = "Rust + WebAssembly ready";
    render();
    input.addEventListener("input", render);
    gfmToggle.addEventListener("change", render);
  } catch (error) {
    console.error("Unable to start the marked-rs playground", error);
    status.textContent = "WebAssembly failed to load";
    output.textContent = "The parser could not be loaded. Please refresh and try again.";
  }
}

start();
