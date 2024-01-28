const { appWindow } = window.__TAURI__.window


window.addEventListener("load", () => {
    let style = document.head.appendChild(document.createElement("style"));
    style.innerHTML = "#my_library_btn:after {background-color: #213280;}";

    let my_library = document.getElementById("my_library_btn")
    let online_library = document.getElementById("online_library_btn")
    my_library.addEventListener("click", () => {
        style.innerHTML = "#online_library_btn:after {background-color: transparent;}";
        style.innerHTML = "#my_library_btn:after {background-color: #213280;}";
        document.getElementById("online_library").style.display = "none"
        document.getElementById("my_library").style.display = "flex"
    })

    online_library.addEventListener("click", () => {
        style.innerHTML = "#my_library_btn:after {background-color: transparent;}";
        style.innerHTML = "#online_library_btn:after {background-color: #213280;}";
        document.getElementById("my_library").style.display = "none"
        document.getElementById("online_library").style.display = "flex"
    })

    document.getElementById("add_wallpaper_btn").addEventListener("click", () => {
        window.location.href = "../../add_wallpaper/html/add_wallpaper.html"
        appWindow.setTitle('Alive Backdrops - Add Wallpaper')
    })

    document.getElementById("settings").addEventListener("click", async () => {
        window.location.href = "../../settings/html/settings.html"

        const { isPermissionGranted, requestPermission, sendNotification } = window.__TAURI__.notification
        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }
        if (permissionGranted) {
            sendNotification('Tauri is awesome!');
            sendNotification({ title: 'TAURI', body: 'Tauri is awesome!' });
        }
    })
})