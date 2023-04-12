import {Button, Card, CardActions, CardContent, FormControl, Input, Typography} from "@mui/material";
// @ts-ignore
import {get} from '@andreekeberg/imagedata';
import {ChangeEvent} from "react";
import {State, useAppDispatch} from "@/store";
import {connect} from "react-redux";
import ImageView from "@/componenets/image_view";
import UploadIcon from '@mui/icons-material/Upload';

export default function ImageUploader() {
  const dispatch = useAppDispatch();

  const blobToPNG = (file: File): Promise<ImageData> => {
    return new Promise((resolve, reject) => {
      // @ts-ignore
      get(file, (err, image) => {
        if (err) {
          reject(err);
        } else {
          resolve(image);
        }
      });
    });
  };

  // turn the file into an image
  const imageToImageData = async (file: File) => {
    return await blobToPNG(file);
  }


  const onChange = async (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    // @ts-ignore
    const file = e.target.files.item(0);


    // @ts-ignore
    const imageData = await imageToImageData(file).catch(console.error);

    dispatch({type: "app/setInitial", payload: imageData});
  }

  const ImageCanvas = connect((state: State) => {
   return {
      image: state.final
   }
  })(ImageView);

  return <Card>
    <CardContent>
      <Typography variant="h4">Image Uploader (PNG only)</Typography>

      <form>
        <FormControl>
          <input
              autoComplete="off"
              type="file" id="image-uploader"
              onChange={onChange}
              accept={"image/png"}
              hidden
          />
          <label htmlFor="image-uploader">
            <Button
                startIcon={<UploadIcon/>}
                variant="outlined" component="span">
              Upload File
            </Button>
          </label>

        </FormControl>
      </form>
    </CardContent>

    <ImageCanvas/>


    {/*<CardActions>*/}
    {/*  <Button variant="outlined">Upload!</Button>*/}
    {/*</CardActions>*/}
  </Card>
}