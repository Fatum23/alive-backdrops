const { invoke } = window.__TAURI__.tauri


window.onload = function () {
    invoke("wallpaper_service")
}

function change(wallpaper, audio) {
    let video = document.getElementById("video")
    if (wallpaper === "true") {
        video.play()
    }
    else {
        video.pause()
    }


    if (audio) {
        video.muted = false
    }
    else {
        video.muted = true;
    }
}