import {Button, Card, CardActions, Grid} from "@mui/material";
import {useAppDispatch} from "@/store";

export default function EdgeForm() {
  const dispatch = useAppDispatch();
  const doLaplacian = () => {
    dispatch({type: "app/addLaplacianEdgeOperation", payload: {}});
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
        <Grid item xs>
          <Button variant="outlined" style={bigButtons}>Add Prewitt To Pipeline</Button>
        </Grid>

        <Grid item xs>
          <Button variant="outlined" style={bigButtons}>Add Sobel To Pipeline</Button>
        </Grid>
        <Grid item xs>
          <Button variant="outlined" style={bigButtons}>Add Sobel To Pipeline</Button>
        </Grid>

        <Grid item xs>
          <Button variant="outlined" style={bigButtons} onClick={doLaplacian}>Add Laplacian To Pipeline</Button>
        </Grid>

        <Grid item xs>
          <Button variant="outlined" style={bigButtons}>Add Laplacian of Gaussian (5x5) To Pipeline</Button>
        </Grid>

      </Grid>
    </CardActions>

  </Card>
}