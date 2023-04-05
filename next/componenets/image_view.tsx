import {useEffect} from "react";
import assert from "assert";


export default async function ImageView(props: { image: ImageData, }) {
    // convert image from ImageData to ImageBitmap
    const bitmap = await createImageBitmap(props.image);

    useEffect(() => {
        const canvas = document.getElementById("image_view_canvas") as HTMLCanvasElement;
        const ctx = canvas.getContext("2d");
        assert(ctx !== null);
        ctx.drawImage(bitmap, 0, 0);
    });

    return <div>
        <canvas id="image_view_canvas" width={bitmap.width} height={bitmap.height}/>
    </div>
}