import {
  Accordion, AccordionDetails,
  AccordionSummary,
  Button,
  Card,
  CardActions,
  FormControl,
  Grid,
  TextField
} from "@mui/material";
import {useAppDispatch} from "@/store";
import React, {ChangeEvent, useState} from "react";
import ExpandMoreIcon from "@mui/icons-material/ExpandMore";

export default function Misc() {
  const dispatch = useAppDispatch();
  const [probability, setProbability] = useState(0.2);

  const onProbabilityChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setProbability(parseFloat(e.target.value));
  }

  const doPepper = () => {
    dispatch({type: "app/addAddPepperOperation", payload: {probability: probability}});
    dispatch({type: "app/runPipeline"});
  }

  const doSalt = () => {
    dispatch({type: "app/addAddSaltOperation", payload: {probability: probability}});
    dispatch({type: "app/runPipeline"});
  }

  const doGreyScale = () => {
    dispatch({type: "app/addGreyScaleOperation"});
    dispatch({type: "app/runPipeline"});
  }

  return <Card>
    <CardActions>
      <Grid container spacing={2}>
        <Grid item xs={12}>
          <Button fullWidth variant="outlined" onClick={doGreyScale}>Add Greyscale to Pipeline</Button>
        </Grid>

        <Grid item xs={12}>
          <Accordion>
            <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
              Noise
            </AccordionSummary>
            <AccordionDetails>
              <Grid container spacing={2}>
                <Grid item xs={12}>
                  <FormControl fullWidth>
                    <TextField
                        type="number"
                        label="Bernouli Trial Probability"
                        value={probability}
                        inputProps={{min: 0, max: 1, step: 0.01}}
                        onChange={onProbabilityChange}></TextField>
                  </FormControl>
                </Grid>

                <Grid item xs>
                  <Button fullWidth variant="outlined" onClick={doSalt}>Add Salt</Button>
                </Grid>

                <Grid item xs>
                  <Button fullWidth variant="outlined" onClick={doPepper}>Add Pepper</Button>
                </Grid>
              </Grid>
            </AccordionDetails>
          </Accordion>
        </Grid>
      </Grid>
    </CardActions>
  </Card>
}