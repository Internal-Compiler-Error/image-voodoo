import {Button, Card, CardActions, CardContent, FormControl, Grid, Paper, TextField, Typography} from "@mui/material";
import {useAppDispatch} from "@/store";
import {ChangeEvent, useState} from "react";

export default function Crop() {
  const [removal, setRemoval] = useState(0);

  const dispatch = useAppDispatch();

  const doCropRight = () => {
    dispatch({type: "app/addCropRightOperation", payload: {removal: removal}});
    dispatch({type: "app/runPipeline"});
  }

  const doCropBottom = () => {
    dispatch({type: "app/addCropBottomOperation", payload: {removal: removal}});
    dispatch({type: "app/runPipeline"});
  }


  const updateRemoval = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setRemoval(parseInt(e.target.value));
  }

  return <Card>
    <CardContent>
      <Typography variant="body1" >
        Only cropping from the left and bottom is directly <em>offered</em> because you can model the others with a
        combination of flip and crop. The spirit of this app is allowing you to compose.
      </Typography>
    </CardContent>

    <CardActions>

      <Grid container spacing={2}>
        <Grid item xs={12}>
          <FormControl fullWidth>
            <TextField
                type={"number"}
                label={"Removal amount"}
                onChange={updateRemoval}
                value={removal}
                InputProps={{inputProps: {min: 0, step: 1}}}
            ></TextField>
          </FormControl>
        </Grid>

        <Grid item xs>
          <Button fullWidth variant="outlined" onClick={doCropRight}>Crop Right</Button>
        </Grid>
        <Grid item xs>
          <Button fullWidth variant="outlined" onClick={doCropBottom}>Crop Bottom</Button>
        </Grid>
      </Grid>
    </CardActions>
  </Card>
}