const { invoke } = window.__TAURI__.tauri


async function readStore() {
    return await invoke("read_store")
}
export async function getStore(key) {
    let store = await readStore()
    let keys = key.split(".")
    let finalKey = keys.pop()
    let node = keys.reduce((node, key) => node[key], JSON.parse(store))
    return node[finalKey]
}
export async function setStore(key, value) {
    let store = JSON.parse(await readStore())
    let keys = key.split(".")
    let finalKey = keys.pop()
    let node = keys.reduce((node, key) => node[key], store)
    node[finalKey] = value
    await invoke("write_store", { content: JSON.stringify(store, null, "\t")})
}
//
// console.log(await getStore("titlebar.window_max"))
// await setStore("titlebar.window_max", true)
// console.log(await getStore("titlebar.window_max"))