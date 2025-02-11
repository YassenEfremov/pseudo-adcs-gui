const { invoke, Channel } = window.__TAURI__.core;

let devicesEl;
let rotationEl;

async function startMain() {

  const onEvent = new Channel();
  const decoder = new TextDecoder();
  onEvent.onmessage = (msgBytes) => {
    const message = decoder.decode(msgBytes);
    rotationEl.innerHTML = message;
  };

  await invoke("telemetry", { onEvent });
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
      await stopMain();
    } catch (error) {
      deviceEl.style.backgroundColor = "red";
    }
  } else {
    try {
      await invoke("connect", { addrStr: deviceEl.dataset.addrStr });
      deviceEl.dataset.connected = "true";
      deviceEl.style.backgroundColor = "lightblue";
      await startMain();
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
  rotationEl = document.querySelector("#rotation");
  document.querySelector("#scan").addEventListener("click", async (e) => {
    e.preventDefault();
    await scan();
  });
});
