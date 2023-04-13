import {ChangeEvent, useState} from "react";
import {
  Button,
  Card,
  CardActions,
  CardContent, FormControl,
  FormLabel,
  Input,
  InputLabel,
  Paper,
  TextField,
  Typography
} from "@mui/material";
import {useAppDispatch} from "@/store";

export default function PowerForm() {
  const dispatch = useAppDispatch();
  const [gamma, setGamma] = useState(0);

  const gammaChange = (e: ChangeEvent<HTMLInputElement>) => {
    setGamma(parseFloat(e.target.value));
  }


  const onSubmit = (e: ChangeEvent<HTMLFormElement>) => {
    e.preventDefault();
    // dispatch  the params to the IHateRedux
    dispatch({type: "app/addPowerOperation", payload: {gamma}});
    dispatch({type: "app/runPipeline"});
  }


  return <Card elevation={3}>
    <CardContent>
      {/*<Typography variant="h4">Power Mapping</Typography>*/}
      <form onSubmit={onSubmit}>
        <FormControl fullWidth>
          <TextField label="γ" onChange={gammaChange} id="power-mappping-gamma" type="number"
                     InputProps={{
                       inputProps: {
                         min: 0,
                         step: 0.005,
                       }
                     }
                     }/>
        </FormControl>

        <CardActions>
          <Button variant="outlined" type="submit">Add To Pipeline</Button>
        </CardActions>
      </form>
    </CardContent>
  </Card>
}
