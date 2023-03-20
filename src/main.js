const { invoke } = window.__TAURI__.tauri;

let collatzInputEl;
let collatzMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  collatzInputEl = document.querySelector("#collatz-input");
  collatzMsgEl = document.querySelector("#collatz-result");
});

async function collatz() {
  let result = await invoke("generate_collatz", { startNumber: parseInt(collatzInputEl.value) });
  collatzMsgEl.textContent = JSON.stringify(result);
  graph(JSON.stringify(result));
}

window.collatz = collatz;
