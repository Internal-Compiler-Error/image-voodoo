import {useEffect} from "react";
import assert from "assert";


export default function ImageView(props: { image: ImageData | undefined, }) {
  const canvasWidth = 400;
  const canvasHeight = 400;

  useEffect(() => {
    const canvas = document.getElementById("image_view_canvas") as HTMLCanvasElement;
    canvas.width = screen.width / 3.7;
    canvas.height = screen.height / 3.7;
  }, []);


  useEffect(() => {
    // This is the most idiotic aspect of React,
    async function draw() {
      assert(props.image !== undefined);

      const {width, height} = props.image;

      // convert image from ImageData to ImageBitmap
      const bitmap = await createImageBitmap(props.image);
      const canvas = document.getElementById("image_view_canvas") as HTMLCanvasElement;

      const scale = Math.max(canvasWidth / width, canvasHeight / height);
      const ctx = canvas.getContext("2d");
      assert(ctx !== null);
      ctx.drawImage(bitmap, 0, 0, width * scale , height * scale);
    }


    if (props.image !== undefined) {
      draw().catch(console.error);
    } else {
      // nothing to draw noop
    }


  }, [props.image])

  return <div>
    <canvas id="image_view_canvas"
            style={{border: "thick dashed red"}}
            width={canvasWidth} height={canvasHeight} >
      Your browser does not support the canvas element. Imagine this to be an image. :)
    </canvas>
  </div>
}