window.addEventListener("load", () => {
    document.addEventListener('keydown', (e) => {
        e = e || window.event;
        let keyCodes =  [114, 116, 118, 123]
        if(keyCodes.includes(e.keyCode) || (e.ctrlKey && e.keyCode === 82)){
            e.preventDefault();
        }
    });
    window.addEventListener("contextmenu", (event) => {
        event.preventDefault()
    })
})