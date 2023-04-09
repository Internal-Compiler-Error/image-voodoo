import {Button, Card, CardActions, CardContent, Grid, Typography} from "@mui/material";

export default function FilteringForm() {
  const bigButtons = {
    width: "100%",
    height: "100%",
  }


  return <Card>
    {/*<CardContent>*/}
    {/*  <Typography variant="body1">*/}
    {/*    Some default filters can be added directly, or you can do it manually using the convolution input form.*/}
    {/*  </Typography>*/}
    {/*</CardContent>*/}
    <CardActions>
      <Grid container
            direction="row"
            justifyContent="space-between"
            alignItems="stretch"
            spacing={2}>

        <Grid item xs>
          <Button variant={"outlined"} style={bigButtons}>Add Min Filter</Button>
        </Grid>

        <Grid item xs>
          <Button variant={"outlined"} style={bigButtons}>Add Median Filter</Button>
        </Grid>

        <Grid item xs>
          <Button variant={"outlined"} style={bigButtons}>Add Max Filter</Button>
        </Grid>

        <Grid item xs>
          <Button variant={"outlined"} style={bigButtons}>Add 3x3 Gaussian</Button>
        </Grid>
      </Grid>

    </CardActions>
  </Card>
}