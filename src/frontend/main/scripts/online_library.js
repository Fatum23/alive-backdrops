window.addEventListener("load", () => {
    function getInternetConnection() {
        let no_wifi = document.getElementById("no_wifi")
        if (navigator.onLine) {
            no_wifi.style.display = "none"
        }
        else {
            no_wifi.style.display = "flex"
        }
    }
    window.addEventListener('online', getInternetConnection)
    window.addEventListener('offline', getInternetConnection)
    getInternetConnection()
})