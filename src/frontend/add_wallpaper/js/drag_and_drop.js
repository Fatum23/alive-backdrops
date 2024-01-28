const { listen } = window.__TAURI__.event
const { convertFileSrc } = window.__TAURI__.tauri


function change_preview(path) {
    path = convertFileSrc(path)
    document.getElementById('preview').src = path
}


window.onload = function () {
    listen('tauri://file-drop', event => {
        let path = event.payload
        change_preview(path)
        document.getElementById("path").innerHTML = event.payload
    });
}