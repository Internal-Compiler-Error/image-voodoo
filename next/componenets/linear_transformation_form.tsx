import {ChangeEvent, useState} from "react";
import {
  Button,
  Card,
  CardActions,
  CardContent, FormControl,
  FormLabel, Grid,
  Input,
  InputLabel,
  Paper,
  TextField,
  Typography
} from "@mui/material";
import {useAppDispatch} from "@/store";

export default function LinearTransformationForm() {
  const dispatch = useAppDispatch();
  const [gain, setGain] = useState(1);
  const [bias, setBias] = useState(0);

  const biasChange = (e: ChangeEvent<HTMLInputElement>) => {
    setBias(parseFloat(e.target.value));
  }

  const gainChange = (e: ChangeEvent<HTMLInputElement>) => {
    setGain(parseFloat(e.target.value));
  }

  const onSubmit = (e: ChangeEvent<HTMLFormElement>) => {
    e.preventDefault();
    // dispatch  the params to the IHateRedux
    dispatch({type: "app/addLinearOperation", payload: {gain, bias}});
  }


  return <Card elevation={3}>
    <CardContent>
      {/*<Typography variant="h4">Linear Transformation</Typography>*/}
      <form onSubmit={onSubmit}>
        <FormControl fullWidth>
          <Grid container spacing={2}>
            <Grid item xs={6}>
              {/*<InputLabel id="linear-transfomration-gain-label">Gain</InputLabel>*/}
              <TextField fullWidth label="Gain" onChange={gainChange} id="linear-transfomration-gain" type="number"
                         value={gain}/>
            </Grid>

            <Grid item xs={6}>
              {/*<InputLabel id="linear-transformation-bias-label">Bias</InputLabel>*/}
              <TextField fullWidth label="Bias" onChange={biasChange} id="linear-transformation-bias" type="number"
                         value={bias}/>
            </Grid>

          </Grid>

        </FormControl>

        <CardActions>
          <Button variant="outlined" type="submit">Add To Pipeline</Button>
        </CardActions>
      </form>
    </CardContent>
  </Card>

}
