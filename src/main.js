const { invoke } = window.__TAURI__.core;

let devicesEl;

async function startMain() {
  // await invoke("telemetry", {});
}

async function stopMain() {

}

async function toggleConnection(event) {
  let deviceEl = event.target;
  if (deviceEl.dataset.connected == "true") {
    try {
      await invoke("disconnect", {});
      deviceEl.dataset.connected = "false";
      deviceEl.style.backgroundColor = "white";
      stopMain();
    } catch (error) {
      deviceEl.style.backgroundColor = "red";
    }
  } else {
    try {
      await invoke("connect", { addrStr: deviceEl.dataset.addrStr });
      deviceEl.dataset.connected = "true";
      deviceEl.style.backgroundColor = "lightblue";
      startMain();
    } catch (error) {
      deviceEl.style.backgroundColor = "red";
    }
  }
}

async function scan() {
  devicesEl.innerHTML = "";
  try {
    let devices = await invoke("scan", {});
    for (const [name, addrStr] of devices) {
      let deviceEl = document.createElement("button");
      deviceEl.innerHTML = name;
      deviceEl.dataset.addrStr = addrStr;
      deviceEl.dataset.connected = "false";
      deviceEl.addEventListener("click", toggleConnection);
      devicesEl.appendChild(deviceEl);
    }
  } catch (error) {
    let errEl = document.createElement("p");
    errEl.innerHTML = "BL not enabled or not supported";
    devicesEl.appendChild(errEl);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  devicesEl = document.querySelector("#devices");
  document.querySelector("#scan").addEventListener("click", (e) => {
    e.preventDefault();
    scan();
  });
});
