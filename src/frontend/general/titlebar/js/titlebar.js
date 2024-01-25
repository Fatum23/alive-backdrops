const { LogicalSize, appWindow } = window.__TAURI__.window;
const { confirm } = window.__TAURI__.dialog
const { invoke } = window.__TAURI__.tauri


window.addEventListener("load", async () => {
    document.getElementById('minimize').addEventListener("click", event => {
        appWindow.minimize()
    });


    let maximize = document.getElementById('maximize')
    let maximize_icon = document.getElementById('maximize_img')
    if (await appWindow.isMaximized()) {
        maximize_icon.src = "../../../assets/images/unmaximize.png"
    }
    else {
        maximize_icon.src = "../../../assets/images/maximize.png"
    }

    maximize.addEventListener("click", async event => {
        if (await appWindow.isMaximized()) {
            await appWindow.setSize(new LogicalSize(800, 600))
            await appWindow.center()
        } else {
            appWindow.maximize()
        }
    });


    await appWindow.onResized(async () => {
        if (await appWindow.isMaximized()) {
            maximize_icon.src = "../../../assets/images/unmaximize.png"
        }
        else {
            maximize_icon.src = "../../../assets/images/maximize.png"
        }
    });

    document.getElementById('close').addEventListener("click", async event => {
        await appWindow.close()
    });
})