const information = document.getElementById('info');
console.log("info: ", information);
information.innerText = `This app is using Chrome (v${versions.chrome()}), Node.js (v${versions.node()}), and Electron (v${versions.electron()})`;


const func = async () => {
    const resp = await window.versions.ping();
    console.log(resp);
}

func();
