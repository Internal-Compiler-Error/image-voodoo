import {Button, Card, CardActions, CardContent, FormControl, Grid, TextField} from "@mui/material";
import {ChangeEvent, useState} from "react";
import {useAppDispatch} from "@/store";

export default function FlipForm() {
  const dispatch = useAppDispatch();

  const flipAlongX = () => {
    dispatch({type: "app/addFlipOperation", payload: {axis: "x"}});
    dispatch({type: "app/runPipeline"});
  }

  const flipAlongY = () => {
    dispatch({type: "app/addFlipOperation", payload: {axis: "y"}});
    dispatch({type: "app/runPipeline"});

  }


  return <Card>
    <CardActions>
      <Grid container
            direction="row"
            spacing={2}
            justifyContent="space-between"
            alignItems="stretch">
        <Grid item xs>
          <Button fullWidth variant="outlined" onClick={flipAlongX}>Flip Along X Axis</Button>
        </Grid>

        <Grid item xs>
          <Button fullWidth variant="outlined" onClick={flipAlongY}>Flip Along Y Axis</Button>
        </Grid>
      </Grid>
    </CardActions>
  </Card>
}