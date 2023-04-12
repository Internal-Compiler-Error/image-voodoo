import {ChangeEvent, useState} from "react";
import {Button, Card, CardActions, FormControl, Grid, TextField} from "@mui/material";


export default function Noise() {
  const [probability, setProbability] = useState(0.2);

  const onProbabilityChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setProbability(parseFloat(e.target.value));
  }


  return <Card>
    <CardActions>
      <Grid container spacing={2}>
        <Grid item xs={12}>
          <FormControl fullWidth>
            <TextField type="number" label="Bernouli Trial Probability" value={probability}
                       onChange={onProbabilityChange}></TextField>
          </FormControl>
        </Grid>

        <Grid item xs>
          <Button fullWidth variant="outlined">Add Salt</Button>
        </Grid>

        <Grid item xs>
          <Button fullWidth variant="outlined">Add Pepper</Button>
        </Grid>
      </Grid>
    </CardActions>
  </Card>
}