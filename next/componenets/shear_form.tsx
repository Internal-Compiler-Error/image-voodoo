import {Button, Card, CardActions, CardContent, FormControl, Grid, TextField} from "@mui/material";
import {ChangeEvent, useState} from "react";

export default function ShearForm() {
  const [xShear, setXShear] = useState(0);
  const [yShear, setYShear] = useState(0);


  const onXShearChange = (e: ChangeEvent<HTMLInputElement>) => {
    setXShear(parseFloat(e.target.value));
  }

  const onYShearChange = (e: ChangeEvent<HTMLInputElement>) => {
    setYShear(parseFloat(e.target.value));
  }

  return <Card>
    <CardContent>
      <Grid container spacing={2}>
        <Grid item xs>
          <FormControl fullWidth>

            <TextField type="number" label="Shear in horizontal" value={xShear} onChange={onXShearChange}/>

          </FormControl>
        </Grid>

        <Grid item xs>
          <FormControl fullWidth>
            <TextField type="number" label="Shear in vertical" value={yShear} onChange={onYShearChange}/>
          </FormControl>
        </Grid>
      </Grid>


    </CardContent>
    <CardActions>
      <Button variant="outlined">Add Shear To Pipeline</Button>
    </CardActions>
  </Card>
}