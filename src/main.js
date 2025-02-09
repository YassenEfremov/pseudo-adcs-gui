const { invoke } = window.__TAURI__.core;

let adapterListEl;

async function scan() {
  adapterListEl.innerHTML = "";
  let adapterList = await invoke("ble", {});
  for (const adapter of adapterList) {
    const childEl = document.createElement("button");
    childEl.textContent = adapter;
    childEl.addEventListener("click", (e) => {
      console.log("but");
    });
    adapterListEl.appendChild(childEl);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  adapterListEl = document.querySelector("#devices");
  document.querySelector("#scan").addEventListener("click", (e) => {
    e.preventDefault();
    scan();
  });
});
