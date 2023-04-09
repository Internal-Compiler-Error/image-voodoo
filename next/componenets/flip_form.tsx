import {Button, Card, CardActions, CardContent, FormControl, Grid, TextField} from "@mui/material";
import {ChangeEvent, useState} from "react";

export default function FlipForm() {
  return <Card>
    <CardActions>
      <Grid container
            direction="row"
            spacing={2}
            justifyContent="space-between"
            alignItems="stretch">
        <Grid item xs>
          <Button fullWidth variant="outlined">Flip Along Horizontal Axis</Button>
        </Grid>

        <Grid item xs>
          <Button fullWidth variant="outlined">Flip Along Vertical Axis</Button>
        </Grid>
      </Grid>
    </CardActions>
  </Card>
}