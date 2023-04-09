import {Button, Card, CardActions, FormControl, Grid, Paper, TextField, Typography} from "@mui/material";
import {useAppDispatch} from "@/store";
import {ChangeEvent, useState} from "react";

export default function EdgeForm() {
  const dispatch = useAppDispatch();
  const [threshold, setThreshold] = useState(200);

  const onThresholdChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setThreshold(parseFloat(e.target.value));
  }

  const doLaplacian = () => {
    dispatch({type: "app/addLaplacianEdgeOperation", payload: {threshold: threshold}});
  }

  const doLaplacianOfGaussian = () => {
    dispatch({type: "app/addLaplacianOfGaussianEdgeOperation", payload: {threshold: threshold}});
  }


  const bigButtons = {
    width: "100%",
    height: "100%",
  }

  return <Card>
    <CardActions>
      <Grid container
            direction="row"
            justifyContent="space-between"
            alignItems="stretch"
            spacing={2}>
        <Grid item xs={12}>
          <Paper elevation={2}>
            <Typography variant="body1" paragraph align="justify">
              All of them use reflective indexing, let&apos;s not kid ourselves here. Zero and circular indexing is
              almost never the right choice when you&apos;re using techniques from signal processing where periodicity
              is expected.
            </Typography>
          </Paper>
        </Grid>

        <Grid item xs={12}>
          <FormControl fullWidth>
            <TextField type="number" label="Threshold" value={threshold} onChange={onThresholdChange}></TextField>
          </FormControl>
        </Grid>


        <Grid item xs>
          <Button variant="outlined" style={bigButtons}>Add Prewitt To Pipeline</Button>
        </Grid>

        <Grid item xs>
          <Button variant="outlined" style={bigButtons}>Add Sobel To Pipeline</Button>
        </Grid>

        <Grid item xs>
          <Button variant="outlined" style={bigButtons} onClick={doLaplacian}>Add Laplacian To Pipeline</Button>
        </Grid>

        <Grid item xs>
          <Button variant="outlined" style={bigButtons} onClick={doLaplacianOfGaussian}>Add Laplacian of Gaussian (5x5)
            To Pipeline</Button>
        </Grid>

      </Grid>
    </CardActions>

  </Card>
}