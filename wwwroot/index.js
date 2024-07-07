import init, { img_canvas, dl_canvas } from './rust_photoeditor.js';

const myCanvas = document.getElementById("myCanvas");
myCanvas.width = 768;
myCanvas.height = 768;
const myRect = myCanvas.getBoundingClientRect();
const myContext = myCanvas.getContext("2d");
myContext.lineWidth = 3;
myContext.fillStyle = "black";
let pressed = false;
const myXY = [];
myCanvas.addEventListener("mousedown", (event) => {
    myContext.beginPath();
    myContext.moveTo(event.offsetX, event.offsetY);
    pressed = true;
});
document.addEventListener("mousemove", (event) => {
    // Canvas要素の位置を取得
    const mouse_x = event.clientX - myRect.left;
    const mouse_y = event.clientY - myRect.top;
    //Canvas外
    if (mouse_x < 0 || mouse_x > myCanvas.clientWidth || mouse_y < 0 || mouse_y > myCanvas.clientHeight) {
        pressed = false;
    }
    if (pressed === true) {
        myContext.lineTo(event.offsetX, event.offsetY);
        myContext.stroke();
        myContext.beginPath();
        myContext.moveTo(event.offsetX, event.offsetY);
    }
});
myCanvas.addEventListener("mouseup", (event) => {
    pressed = false;
    myContext.lineTo(event.offsetX, event.offsetY);
    myContext.stroke();
});
myCanvas.addEventListener("touchmove", (event) => {
    // タッチ座標をCanvas座標系に変換
    let canvas_x = event.touches[0].clientX - myRect.left;
    let canvas_y = event.touches[0].clientY - myRect.top;
    // 現在の位置に矩形を描画
    myContext.fillRect(canvas_x, canvas_y, 1, 1);
    myXY.push({ x: canvas_x, y: canvas_y });
});
myCanvas.addEventListener("touchend", (event) => {
    for (let i = 0; i < myXY.length; i++) {
        if (i > 0 && i < myXY.length - 1) {
            myContext.beginPath();
            myContext.moveTo(myXY[i - 1].x, myXY[i - 1].y);
            myContext.lineTo(myXY[i].x, myXY[i].y);
            myContext.stroke();
        }
    }
    myXY.length = 0;
});


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
                const ret = img_canvas(uint8Array, myCanvas.clientWidth);
                console.log(ret);
                myCanvas.width = ret.resize_w;
                myCanvas.height = ret.resize_h;
                myCanvas.style.width = ret.resize_w + "px";
                myCanvas.style.height = ret.resize_h + "px";
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
        const file_blob = new Blob([files[0]], { type: files[0].type });
        await blobToUint8Array(file_blob)
            .then(uint8Array => {
                dl_canvas(dataURLtoUint8Array(myCanvas), uint8Array);
            })
            .catch(error => {
                console.error('Error converting blob:', error);
            });
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