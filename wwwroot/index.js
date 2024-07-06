import init, { img_canvas, dl_canvas } from './rust_photoeditor.js';

async function run() {
    await init();

    document.getElementById("file_input").addEventListener("change", async (event) => {
        const files = document.getElementById("file_input").files;
        if (files.length === 0) {
            window.confirm("err:Fileをセットしてください");
            return;
        }
        const file_blob = new Blob([files[0]], { type: files[0].type });
        await blobToUint8Array(file_blob)
            .then(uint8Array => {
                img_canvas(uint8Array);
            })
            .catch(error => {
                console.error('Error converting blob:', error);
            });
    });
    document.getElementById("dl_btn").addEventListener("click", async () => {
        const files = document.getElementById("file_input").files;
        if (files.length === 0) {
            window.confirm("err:Fileをセットしてください");
            return;
        }
        const canvas = document.getElementById("myCanvas");
        dl_canvas(dataURLtoUint8Array(canvas));
    })
}
run();

function dataURLtoUint8Array(canvas) {
    const dataURL = canvas.toDataURL("image/png");
    const base64Data = dataURL.replace(/^data:image\/png;base64,/, "");
    const binaryData = atob(base64Data);
    const uint8Array = new Uint8Array(binaryData.length);
    for (let i = 0; i < binaryData.length; i++) {
        uint8Array[i] = binaryData.charCodeAt(i);
    }
    return uint8Array;
}
async function blobToUint8Array(blob) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = () => {
            resolve(new Uint8Array(reader.result));
        };
        reader.onerror = reject;
        reader.readAsArrayBuffer(blob);
    });
}