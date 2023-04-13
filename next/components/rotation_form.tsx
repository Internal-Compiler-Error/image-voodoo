import {
  Button,
  Card,
  CardActions,
  CardContent,
  FormControl,
  Grid,
  InputLabel,
  Slider,
  TextField,
  Typography
} from "@mui/material";
import {ChangeEvent, useState} from "react";
import {useAppDispatch} from "@/store";

export default function RotationForm() {
  const [degrees, setDegrees] = useState<number>(0);
  const dispatch = useAppDispatch();

  const onSliderChange = (e: Event, newValue: number | number[]) => {
    setDegrees(newValue as number);
  }

  const onClick = () => {

    dispatch({type: "app/addRotationOperation", payload: {angle: degrees}});
    dispatch({type: "app/runPipeline"});
  }

  const onTextFieldChange = (e: ChangeEvent<HTMLInputElement>) => {
    const degrees = parseFloat(e.target.value);

    // adjust the value to be in the range [0, 360]
    if (degrees > 360) {
      setDegrees(degrees % 360);
    } else if (degrees < 0) {
      setDegrees(360 - (Math.abs(degrees) % 360));
    } else {
      setDegrees(parseFloat(e.target.value));
    }
  }


  return <Card>
    <CardContent>
      <Grid container spacing={4} alignContent="center">
        <Grid item xs>
          <Slider
              aria-label={"Degrees"}
              defaultValue={0}
              value={degrees}
              min={0} max={360} step={0.1}
              marks={
                [
                  {value: 0, label: "0"},
                  {value: 90, label: "90"},
                  {value: 180, label: "180"},
                  {value: 270, label: "270"},
                  {value: 360, label: "360"},
                ]
              }
              valueLabelDisplay="auto"

              onChange={onSliderChange}
          />
        </Grid>

        <Grid item>
          <TextField
              type={"number"}
              value={degrees}
              onChange={onTextFieldChange}
              inputProps={{
                step: 0.1,
              }}
          />
        </Grid>

      </Grid>
    </CardContent>
    <CardActions>
      <Button variant="outlined" onClick={onClick}>Add To Pipeline</Button>
    </CardActions>
  </Card>
}