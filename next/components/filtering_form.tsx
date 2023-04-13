import {Button, Card, CardActions, CardContent, FormControl, Grid, TextField, Typography} from "@mui/material";
import {ChangeEvent, useState} from "react";
import {useAppDispatch} from "@/store";

export default function FilteringForm() {
  const bigButtons = {
    width: "100%",
    height: "100%",
  }

  const [distance, setDistance] = useState(1);
  const dispatch = useAppDispatch();
  const onDistanceChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setDistance(parseInt(e.target.value));
  }

  const doMinFilter = () => {
    dispatch({type: "app/addMinFilterOperation", payload: {distance: distance}});
    dispatch({type: "app/runPipeline"});
  }

  const doMedianFilter = () => {
    dispatch({type: "app/addMedianFilterOperation", payload: {distance: distance}});
    dispatch({type: "app/runPipeline"});
  }

  const doMaxFilter = () => {
    dispatch({type: "app/addMaxFilterOperation", payload: {distance: distance}});
    dispatch({type: "app/runPipeline"});
  }


  return <Card>
    <CardActions>
      <Grid container
            direction="row"
            justifyContent="space-between"
            alignItems="stretch"
            spacing={2}>

        <Grid item xs={12}>
          <FormControl fullWidth>
            <TextField type="number" label="Distance" variant="outlined" value={distance}
                       InputProps={{
                         inputProps: {min: "1", step: "1"},
                       }}
                       onChange={onDistanceChange}
            />
          </FormControl>
        </Grid>

        <Grid item xs>
          <Button variant={"outlined"} style={bigButtons} onClick={doMinFilter}>Add Min Filter</Button>
        </Grid>

        <Grid item xs>
          <Button variant={"outlined"} style={bigButtons} onClick={doMedianFilter}>Add Median Filter</Button>
        </Grid>

        <Grid item xs>
          <Button variant={"outlined"} style={bigButtons} onClick={doMaxFilter}>Add Max Filter</Button>
        </Grid>
      </Grid>

    </CardActions>
  </Card>
}