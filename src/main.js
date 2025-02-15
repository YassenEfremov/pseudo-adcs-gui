// import * as THREE from 'three';
const { invoke, Channel } = window.__TAURI__.core;

let devicesEl = document.querySelector("#devices-hidden");
let devicesPlaceholderEl = document.querySelector("#devices-placeholder");
let devicesLoadingEl = document.querySelector("#devices-loading");
let devicesErrorEl = document.querySelector("#devices-error");

let mainEl = document.querySelector("#main-hidden");
let mainPlaceholderEl = document.querySelector("#main-placeholder");
let mainLoadingEl = document.querySelector("#main-loading");
let mainErrorEl = document.querySelector("#main-error");

const inputXEl = document.querySelector("#input-x");
const inputYEl = document.querySelector("#input-y");
const inputZEl = document.querySelector("#input-z");

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
  let deviceAddrEl = document.createElement("p");
  deviceAddrEl.innerHTML = address;

  deviceEl.appendChild(deviceNameEl);
  deviceEl.appendChild(deviceAddrEl);
  return deviceEl;
}


async function startMain() {
  const onEvent = new Channel();
  const textDecoder = new TextDecoder();
  let angleXEl = document.querySelector("#angle-x");
  let angleYEl = document.querySelector("#angle-y");
  let angleZEl = document.querySelector("#angle-z");
  let accXEl = document.querySelector("#acc-x");
  let accYEl = document.querySelector("#acc-y");
  let accZEl = document.querySelector("#acc-z");
  onEvent.onmessage = async (msgBytes) => {
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

// async function stopMain() {

// }

async function toggleConnection(event) {
  let deviceEl = event.target;
  mainErrorEl.style.display = "none";
  if (deviceEl.dataset.connected == "true") {
    try {
      mainEl.style.display = "none";
      mainLoadingEl.style.display = "block";
      await invoke("disconnect", {});
      deviceEl.dataset.connected = "false";
      deviceEl.classList.remove("device-button-connected");
      deviceEl.classList.add("device-button");

      mainLoadingEl.style.display = "none";
      mainPlaceholderEl.style.display = "flex";
      // await stopMain();
    } catch (error) {
      mainLoadingEl.style.display = "none";
      mainErrorEl.style.display = "block";
    }
  } else {
    try {
      mainPlaceholderEl.style.display = "none";
      mainLoadingEl.style.display = "block";
      await invoke("connect", { addrStr: deviceEl.dataset.addrStr });
      deviceEl.dataset.connected = "true";
      deviceEl.classList.remove("device-button");
      deviceEl.classList.add("device-button-connected");

      mainLoadingEl.style.display = "none";
      mainEl.style.display = "flex";
      await startMain();
    } catch (error) {
      mainLoadingEl.style.display = "none";
      mainErrorEl.style.display = "block";
    }
  }
}

async function scan() {
  devicesEl.innerHTML = "";
  try {
    devicesEl.style.display = "none";
    devicesPlaceholderEl.style.display = "none";
    devicesLoadingEl.style.display = "block";
    let devices = await invoke("scan", {});
    devicesLoadingEl.style.display = "none";
    devicesEl.style.display = "flex";
    for (const [name, addrStr] of devices) {
      let deviceEl = createDeviceEl(name, addrStr)
      deviceEl.dataset.addrStr = addrStr;
      deviceEl.dataset.connected = "false";
      deviceEl.addEventListener("click", toggleConnection);
      devicesEl.appendChild(deviceEl);
    }
  } catch (error) {
    devicesLoadingEl.style.display = "none";
    devicesErrorEl.style.display = "block";
  }
}

async function setAttitude() {
  const newX = parseInt(inputXEl.value);
  const newY = parseInt(inputYEl.value);
  const newZ = parseInt(inputZEl.value);

  await invoke("set_attitude", { newX: newX, newY: newY, newZ: newZ });
}


window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#scan").addEventListener("click", async (e) => {
    e.preventDefault();
    await scan();
  });

  document.querySelector("#attitude-form").addEventListener("submit", async (e) => {
    e.preventDefault();
    await setAttitude();
  });

  // const scene = new THREE.Scene();
  // const camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );
  
  // const renderer = new THREE.WebGLRenderer();
  // renderer.setSize( window.innerWidth, window.innerHeight );
  // // renderer.setAnimationLoop( animate );
  // document.querySelector("#three-dim-view").appendChild( renderer.domElement );
  
  // const geometry = new THREE.BoxGeometry( 1, 1, 1 );
  // const material = new THREE.MeshBasicMaterial( { color: 0x00ff00 } );
  // const cube = new THREE.Mesh( geometry, material );
  // scene.add( cube );
  
  // camera.position.z = 5;
  
  // function animate() {
  
  //   cube.rotation.x += 0.01;
  //   cube.rotation.y += 0.01;
  
  //   renderer.render( scene, camera );
  
  // }
});
