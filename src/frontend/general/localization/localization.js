// window.store.set("a", "Hello World!")
// window.store.get("a", (event, args) => {
//     alert(args)
// })

window.ipc.send("getLocale")
window.ipc.on("replyLocale", (event, locale) => {
    let user_locale = locale[0]
    if (user_locale !== "ru") {
        user_locale = "en"
    }
})