import {useEffect} from "react";
import assert from "assert";
import {useAppDispatch} from "@/store";
import {blobToPNG} from "@/componenets/image_uploader";


export default function ImageView(props: { image: ImageData | undefined, }) {
  const dispatch = useAppDispatch();

  // set the canvas size to the size of roughly 95% of the parent
  useEffect(() => {
    // const canvas = document.getElementById("image_view_canvas") as HTMLCanvasElement;
    // // @ts-ignore
    // const width = canvas.parentElement?.clientWidth / 1.05;
    //
    // if (width) {
    //   canvas.width = width;
    //   canvas.height = width;
    // }

    const canvas = document.getElementById("image_view_canvas") as HTMLCanvasElement;
    const context = canvas.getContext("2d") as CanvasRenderingContext2D;
    context.imageSmoothingEnabled = false;

  }, []);


  useEffect(() => {
    // This is the most idiotic aspect of React,
    async function draw() {
      assert(props.image !== undefined);

      const {width, height} = props.image;

      // convert image from ImageData to ImageBitmap
      const bitmap = await createImageBitmap(props.image);
      const canvas = document.getElementById("image_view_canvas") as HTMLCanvasElement;
      canvas.width = width;
      canvas.height = height;


      const ctx = canvas.getContext("2d");
      assert(ctx !== null);

      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.drawImage(bitmap, 0, 0);
    }


    // draw the image if it exists, otherwise display a meme
    if (props.image !== undefined) {
      draw().catch(console.error);
    } else {
      // yes this hack is very disgusting
      fetch("https://i.imgur.com/kxT3ugk.png")
          .then(response => response.blob())
          .then(blob => blobToPNG(blob))
          .then((blob) => {
            dispatch({type: "app/setInitial", payload: blob})
          })
    }
  }, [dispatch, props.image])

  return <canvas id="image_view_canvas"
                 style={{
                   border: "medium dashed red",
                   width: "100%",
                   maxWidth: "100%",
                   maxHeight: "100%",
                 }}>
    Your browser does not support the canvas element. Imagine this to be an image. :)
  </canvas>
}