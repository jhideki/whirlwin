const { invoke } = window.__TAURI__.tauri;

let startButton;
let configData;
let programShortcuts;

async function start() {
    startButton.textContent = await invoke("manage_core");
}

async function loadData(){
    configData = await invoke("load_shortcut_data");
    console.log(configData);
    programShortcuts = JSON.parse(configData)["programs"];
    const container = document.getElementById("shortcutContainer");
    container.innerHTML = "";
    for (let i = 0; i < programShortcuts.length; i++){
        if(programShortcuts[i] !== ""){
            const textNode = document.createTextNode("Shorcut " + (i+1) + ": " + programShortcuts[i]);
            const div = document.createElement("div");
            div.appendChild(textNode);
            div.classList.add("shortcut-text");
            container.appendChild(div);
        }
    }
}

loadData();

window.addEventListener("DOMContentLoaded", () => {
    startButton = document.getElementById("startButton");
    const shortcutButton = document.querySelectorAll(".shortcut-button");
    startButton.addEventListener("click", function () {
        console.log("starting program");
        start();
    });
    shortcutButton.forEach(function (element) {
        const button = element.querySelector("button");
        const id = button.id.substring("shortcutButton".length);
        button.addEventListener("click", function () {
            invoke("set_shortcut", { shortcutId: id });
            loadData();
        });
    });
});
