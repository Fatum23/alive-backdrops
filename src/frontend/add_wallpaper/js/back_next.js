const { LogicalSize, appWindow } = window.__TAURI__.window;


window.onload = function () {
    let state = 1
    document.getElementById("back").addEventListener("click", () => { back(state) })
}

function back(state) {
    if (state === 1) {
        window.location.href = "../../main/html/main.html"
        appWindow.close()
    }
}