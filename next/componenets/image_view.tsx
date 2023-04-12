import {useEffect} from "react";
import assert from "assert";


export default function ImageView(props: { image: ImageData | undefined, }) {
  useEffect(() => {
    const canvas = document.getElementById("image_view_canvas") as HTMLCanvasElement;

    // @ts-ignore
    const width = canvas.parentElement?.clientWidth / 1.05;
    const height = width;

    if (width && height) {
      canvas.width = width;
      canvas.height = height;
    }
  }, []);


  useEffect(() => {
    // This is the most idiotic aspect of React,
    async function draw() {
      assert(props.image !== undefined);

      const {width, height} = props.image;

      // convert image from ImageData to ImageBitmap
      const bitmap = await createImageBitmap(props.image);
      const canvas = document.getElementById("image_view_canvas") as HTMLCanvasElement;
      const canvasWidth = canvas.width;
      const canvasHeight = canvas.height;

      const scale = Math.min(canvasWidth / width, canvasHeight / height);
      console.log({width, height, scale, canvasWidth, canvasHeight})

      const ctx = canvas.getContext("2d");
      assert(ctx !== null);
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.drawImage(bitmap, 0, 0, width * scale, height * scale);
    }


    if (props.image !== undefined) {
      draw().catch(console.error);
    } else {
      // nothing to draw noop
    }


  }, [props.image])

  return <div>
    <canvas id="image_view_canvas"
            style={{border: "thick dashed red"}}>
      Your browser does not support the canvas element. Imagine this to be an image. :)
    </canvas>
  </div>
}