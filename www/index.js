import {linear_transformation, gamma_transformation, Kernel, BorderStrategy, convolve, rotate} from "wasm-image-voodoo"
// import * as events from "events";
//
//
// // update the span with the value of the slider
// document.getElementById("gain").oninput = () => {
//     document.getElementById("gainSpan").innerText = document.getElementById("gain").value;
// }
//
// document.getElementById("bias").oninput = () => {
//     document.getElementById("biasSpan").innerText = document.getElementById("bias").value;
// }
//
// document.getElementById("gamma").oninput = () => {
//     document.getElementById("gammaSpan").innerText = document.getElementById("gamma").value;
// }
//
//
// const camera = document.getElementById("cameraFeed");
// const cameraCanvas = document.getElementById("liveCanvas");
// const cameraCtx = cameraCanvas.getContext('2d', {alpha: true});
//
// const processedCanvas = document.getElementById("processedCanvas");
// const processedCtx = processedCanvas.getContext('2d', {alpha: true});
//
// const fps = 120;
//
// // put what the camera captures into the video element
// if (camera) {
//     const dimensions = Math.min(screen.width, screen.height) / 3;
//
//     navigator
//         .mediaDevices
//         .getUserMedia({
//             audio: false,
//             video: {
//                 width: {min: dimensions},
//                 height: {min: dimensions},
//             },
//         })
//         .then(mediaStream => {
//             // get the width and height of the media stream
//             const {width, height} = mediaStream.getVideoTracks()[0].getSettings();
//
//             // set the cameraCanvas size match the media stream size
//             cameraCanvas.style.width = width + "px";
//             cameraCanvas.style.height = height + "px";
//
//             // set the processedCanvas size match the media stream size
//             processedCanvas.style.width = width + "px";
//             processedCanvas.style.height = height + "px";
//
//             return mediaStream;
//         })
//         .then(stream => {
//             camera.srcObject = stream;
//             camera.play();
//         })
//         .catch(e => console.error("error while trying to grab camera" + e));
// }
//
// async function processPipeline(video) {
//     // draw the video to the canvas so we can extract a frame
//     cameraCtx.drawImage(video, 0, 0, cameraCanvas.width, cameraCanvas.height);
//     const imageData = cameraCtx.getImageData(0, 0, cameraCanvas.width, cameraCanvas.height);
//
//     // get the gain and bias from the sliders to perform a linear transformation
//     const gain = document.getElementById("gain").value;
//     const bias = document.getElementById("bias").value;
//     const linearlyTransformed = linear_transformation(imageData, gain, bias)
//
//     // get the gamma from the slider to perform a gamma correction
//     const gamma = document.getElementById("gamma").value;
//     const gammaTransformed = gamma_transformation(linearlyTransformed, gamma);
//
//     // this is the most janky part by far, we need to collect all the input tags to form a kernel
//     // and then pass it to the wasm function
//     const inputs = [];
//     const kernelCells = document.getElementById("kernel");
//     for (let i = 0; i < kernelCells.children.length; i++) {
//         const child = kernelCells.children[i];
//
//         if (child.tagName === "INPUT") {
//             inputs.push(child.value);
//         }
//     }
//
//     const kernel = Kernel.from_vec(Float64Array.from(inputs), 3, 3);
//
//     const convolved = convolve(gammaTransformed, kernel, BorderStrategy.Zero);
//
//
//     // convert the image data into a bitmap
//     const bitmap = await createImageBitmap(convolved)
//
//
//     processedCtx.drawImage(bitmap, 0, 0, cameraCanvas.width, cameraCanvas.height);
// }
//
// const canvasInterval = window.setInterval(() => processPipeline(camera), 1000 / fps);

const form = document.getElementById("image-upload");
form.addEventListener("change", (e) => {
    e.preventDefault(); // prevent the form from submitting
    moveImageToCanvas();
});

const degreeForm = document.getElementById("degrees");
degreeForm.addEventListener("submit", (e) => {
    e.preventDefault();

    let degrees = document.getElementById("degree-input").value;
    let canvas = document.getElementById("upload-canvas");

    let ctx = canvas.getContext("2d");
    let imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    let rotated = rotate(imageData, degrees);


    saveToFile(rotated);


    let bitmap = createImageBitmap(rotated);


    bitmap.then((bitmap) => {
        // clear the canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // make sure the canvas is the same size as the bitmap
        canvas.width = bitmap.width;
        canvas.height = bitmap.height;

        // draw the bitmap to the canva
        ctx.drawImage(bitmap, 0, 0, canvas.width, canvas.height);
    })
});

function saveToFile(image) {
    // save the bitmap to a file
    const canvas2 = document.createElement('canvas');
    const ctx2 = canvas2.getContext('2d');

    // Set the canvas dimensions to the bitmap dimensions
    canvas2.width = image.width;
    canvas2.height = image.height;

    // Draw the bitmap onto the canvas
    ctx2.putImageData(image, 0, 0);

    // Convert the canvas to a data URL
    const dataUrl = canvas2.toDataURL('image/png');

    // Create a link element
    const link = document.createElement('a');

    // Set the link element's attributes
    link.download = 'image.png';
    link.href = dataUrl;

    // Add the link element to the DOM
    document.body.appendChild(link);

    // Click the link element to trigger the download
    link.click();

    // Remove the link element from the DOM
    document.body.removeChild(link);
}

function moveImageToCanvas() {
    const form = document.getElementById("image-input");
    const image = form.files[0];
    const reader = new FileReader();
    const outputCanvas = document.getElementById("upload-canvas");

    // put the image into the canvas
    reader.addEventListener('load', () => {
        // Create a new image element
        const img = new Image();

        // Set the source of the image element to the uploaded file
        img.src = reader.result;

        // When the image has loaded, draw it onto the canvas
        img.addEventListener('load', () => {
            outputCanvas.width = img.width;
            outputCanvas.height = img.height;
            let ctx = outputCanvas.getContext('2d');
            ctx.drawImage(img, 0, 0);
        });
    });

    // Read the uploaded file as a data URL
    reader.readAsDataURL(image);

}