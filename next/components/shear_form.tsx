import {Button, Card, CardActions, CardContent, FormControl, Grid, TextField} from "@mui/material";
import {ChangeEvent, useState} from "react";
import {useAppDispatch} from "@/store";

export default function ShearForm() {
  const [lambda, setLambda] = useState(0);
  const [miu, setMiu] = useState(0);
  const dispatch = useAppDispatch();


  const onLambdaChange = (e: ChangeEvent<HTMLInputElement>) => {
    setLambda(parseFloat(e.target.value));
  }

  const onMiuChange = (e: ChangeEvent<HTMLInputElement>) => {
    setMiu(parseFloat(e.target.value));
  }

  const doShear = () => {
    dispatch({type: "app/addShearOperation", payload: {lambda: lambda, miu: miu}});
    dispatch({type: "app/runPipeline"});
  }


  return <Card>
    <CardContent>
      <Grid container spacing={2}>
        <Grid item xs>
          <FormControl fullWidth>
            <TextField type="number" label="λ" value={lambda} onChange={onLambdaChange}/>
          </FormControl>
        </Grid>

        <Grid item xs>
          <FormControl fullWidth>
            <TextField type="number" label="μ" value={miu} onChange={onMiuChange}/>
          </FormControl>
        </Grid>
      </Grid>


    </CardContent>
    <CardActions>
      <Button variant="outlined" onClick={doShear}>Add Shear To Pipeline</Button>
    </CardActions>
  </Card>
}