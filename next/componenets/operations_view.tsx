import {
  PowerOperation,
  LinearOperation,
  Operation,
  RotationOperation,
  ConvolutionOperation,
  FlipOperation, useAppDispatch
} from "@/store";
import {Box, Button, Card, CardActions, CardContent, Grid, Typography} from "@mui/material";
import assert from "assert";

function PowerOperationCard(props: { operation: PowerOperation }) {
  // assert(props.operation.variant === "Power")

  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Power</Typography>
      <Typography variant="h6">Gamma: {props.operation.gamma}</Typography>
    </CardContent>
  </Card>
}

function LinearOperationCard(props: { operation: LinearOperation }) {
  // assert(props.operation.variant === "Linear")

  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Linear</Typography>
      <Typography variant="h6">scale: {props.operation.gain}</Typography>
      <Typography variant="h6">bias: {props.operation.bias}</Typography>
    </CardContent>
  </Card>
}

function RotationOperationCard(props: { operation: RotationOperation }) {
  // assert(props.operation.variant === "Rotation")

  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Rotation</Typography>
      <Typography variant="h6">Angle: {props.operation.angle}</Typography>
    </CardContent>
  </Card>
}

function ConvolutionOperationCard(props: { operation: ConvolutionOperation }) {
  // assert(props.operation.variant === "Convolution")

  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Convolution</Typography>
      <Typography variant="h6">kernel: {props.operation.kernel}</Typography>
    </CardContent>
  </Card>
}

function OperationCard(props: { operation: Operation }) {
  switch (props.operation.variant) {
    case "Power":
      return <PowerOperationCard operation={props.operation}/>
    case "Linear":
      return <LinearOperationCard operation={props.operation}/>
    case "Rotation":
      return <RotationOperationCard operation={props.operation}/>
    case "Convolution":
      return <ConvolutionOperationCard operation={props.operation}/>
    case "Flip":
      return <FlipOperationCard operation={props.operation}/>
  }
}

function FlipOperationCard(props: { operation: FlipOperation }) {
  // assert(props.operation.variant === "Flip")

  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Flip</Typography>
      <Typography variant="h6">axis: {props.operation.axis}</Typography>
    </CardContent>
  </Card>
}


export default function OperationsView(props: { operations: Operation[] }) {
  const dispatch = useAppDispatch();
  const popLast = () => {
    dispatch({type: "app/removeOperation"});
  }


  return <Card elevation={3}>

    <CardContent>
      <Typography variant="h4">Operations</Typography>
      <CardActions>
        <Button variant="outlined" onClick={popLast}>Pop Last</Button>
      </CardActions>
      <Grid
          container
          direction="row"
          justifyContent="flex-start"
          alignItems="flex-start"
          spacing={1}>
        {props.operations.map((operation, index) =>
            <Grid item key={index}>
              <OperationCard operation={operation}/>
            </Grid>)}
      </Grid>
    </CardContent>

  </Card>
}