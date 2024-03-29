const { invoke } = window.__TAURI__.tauri;

let startButton;
async function start() {
    startButton.textContent = await invoke("manage_core");
}

window.addEventListener("DOMContentLoaded", () => {
    startButton = document.getElementById("startButton");
    const shortcutButton = document.querySelectorAll(".shortcut-button");
    startButton.addEventListener("click", function () {
        console.log("starting program");
        start();
    });
    shortcutButton.forEach(function (element) {
        const button = element.querySelector("button");
        button.addEventListener("click", function () {
            const id = button.id.substring("shortcutButton".length);
            invoke("set_shortcut", { shortcutId: id });
        });
    });
});
