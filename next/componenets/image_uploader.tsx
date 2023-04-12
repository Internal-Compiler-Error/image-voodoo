import {Button, Card, CardActions, CardContent, FormControl, Input, Typography} from "@mui/material";
// @ts-ignore
import {get} from '@andreekeberg/imagedata';
import {ChangeEvent} from "react";
import {State, useAppDispatch} from "@/store";
import {connect} from "react-redux";
import ImageView from "@/componenets/image_view";
import UploadIcon from '@mui/icons-material/Upload';


export const blobToPNG = (file: any): Promise<ImageData> => {
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

export default function ImageUploader() {
  const dispatch = useAppDispatch();

  // turn the file into an image
  const imageToImageData = async (file: File) => {
    return await blobToPNG(file);
  }


  const onSetFile = async (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {

    // @ts-ignore
    if (!e.target.files) {
      return;
    }

    // @ts-ignore
    const file = e.target.files.item(0);
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
      {/*<Typography variant="h4">Image Uploader</Typography>*/}
    </CardContent>

    <CardActions>
      <form>
        <FormControl>
          <input
              autoComplete="off"
              type="file" id="image-uploader"
              onChange={onSetFile}
              accept={"image/png"}
              hidden
          />
          <label htmlFor="image-uploader">
            <Button
                startIcon={<UploadIcon/>}
                variant="outlined" component="span">
              Upload File (PNG only)
            </Button>
          </label>

        </FormControl>
      </form>
    </CardActions>

    <ImageCanvas/>
  </Card>
}