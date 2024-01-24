window.addEventListener("load", () => {
    document.getElementById("back").addEventListener("click", () => {
        window.ipc.send("main")
    })
})