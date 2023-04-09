import {Card, CardContent, CardActions, FormControl, Grid, TextField, Button} from "@mui/material";
import {ChangeEvent, useState} from "react";

export default function ScaleForm() {
  const [xScale, setXScale] = useState(1);
  const [yScale, setYScale] = useState(1);

  const onXScaleChange = (e: ChangeEvent<HTMLInputElement>) => {
    setXScale(parseFloat(e.target.value));
  }

  const onYScaleChange = (e: ChangeEvent<HTMLInputElement>) => {
    setYScale(parseFloat(e.target.value));
  }


  return <Card>
    <CardContent>

      <Grid container spacing={2} direction="row">

        <Grid item xs>
          <FormControl fullWidth>
            <TextField type="number" label="Scale in horizontal" value={xScale} onChange={onXScaleChange}/>
          </FormControl>
        </Grid>

        <Grid item xs>
          <FormControl fullWidth>
            <TextField type="number" label="Scale in vertical" value={xScale} onChange={onYScaleChange}/>
          </FormControl>
        </Grid>
      </Grid>
    </CardContent>
    <CardActions>
      <Button variant="outlined">Add to Pipeline</Button>
    </CardActions>
  </Card>
}