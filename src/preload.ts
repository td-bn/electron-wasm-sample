import { contextBridge, ipcRenderer } from 'electron';

declare global {
    var exposed: Exposed
}

let versions: Versions = {
  node: () => process.versions.node,
  chrome: () => process.versions.chrome,
  electron: () => process.versions.electron,
}

let exposed: Exposed = {
  versions,
  ping: () => ipcRenderer.invoke('ping'),
  greet: (s: string) => ipcRenderer.invoke('greet', s)
}

contextBridge.exposeInMainWorld('exposed', exposed);
