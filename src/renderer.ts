const exposedFns = exposed as Exposed;

const information = document.getElementById('info');
console.log("info: ", information);
const versions = exposedFns.versions;
information.innerText = `This app is using Chrome (v${versions.chrome()}), \
    Node.js (v${versions.node()}), and Electron (v${versions.electron()})`;


const func = async () => {
    const resp = await exposedFns.ping();
    console.log(resp);
    const res = await exposedFns.greet("world");
    alert(res);
}

func();

