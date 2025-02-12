const { invoke, Channel } = window.__TAURI__.core;

let devicesEl;

/**
 * Create a button for a device
 * @param {String} name the device name
 * @param {String} address the device address
 * @returns A button that looks like this:
 * ```
 * <button class="device-button">
 *   <h4> name </h4>
 *   <p> address </p>
 * </button>
 * ```
 */
function createDeviceEl(name, address) {
  let deviceEl = document.createElement("button");
  deviceEl.classList.add("device-button");
  let deviceNameEl = document.createElement("h4");
  if (name) {
    deviceNameEl.innerHTML = name;
  } else {
    deviceNameEl.innerHTML = "unknown";
    deviceNameEl.style.color = "gray";
  }
  deviceNameEl.style.pointerEvents = "none";
  let deviceAddrEl = document.createElement("p");
  deviceAddrEl.innerHTML = address;
  deviceAddrEl.style.pointerEvents = "none";

  deviceEl.appendChild(deviceNameEl);
  deviceEl.appendChild(deviceAddrEl);
  return deviceEl;
}


async function startMain() {
  document.querySelector("#main-placeholder").style.display = "none";
  document.querySelector("#main-hidden").style.display = "flex";

  const onEvent = new Channel();
  const textDecoder = new TextDecoder();
  let angleXEl = document.querySelector("#angle-x");
  let angleYEl = document.querySelector("#angle-y");
  let angleZEl = document.querySelector("#angle-z");
  let accXEl = document.querySelector("#acc-x");
  let accYEl = document.querySelector("#acc-y");
  let accZEl = document.querySelector("#acc-z");
  onEvent.onmessage = (msgBytes) => {
    const values = JSON.parse(textDecoder.decode(msgBytes));
    console.log(values);
    angleXEl.innerHTML = values.x.angle;
    angleYEl.innerHTML = values.y.angle;
    angleZEl.innerHTML = values.z.angle;
    accXEl.innerHTML = values.x.acc;
    accYEl.innerHTML = values.y.acc;
    accZEl.innerHTML = values.z.acc;
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
      deviceEl.style.border = "2px solid white";
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
      let deviceEl = createDeviceEl(name, addrStr)
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
  document.querySelector("#scan").addEventListener("click", async (e) => {
    e.preventDefault();
    await scan();
  });
});
