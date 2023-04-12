import {
  PowerOperation,
  LinearOperation,
  Operation,
  RotationOperation,
  ConvolutionOperation,
  FlipOperation,
  useAppDispatch,
  ShearOpration,
  ScaleOperation,
  EqualizeOperation,
  MinFilterOperation,
  MedianFilterOperation,
  MaxFilterOperation,
  GreyScaleOperation,
  CropRightOperation,
  CropBottomOperation,
  AddSaltOperation,
  AddPepperOperation,
  SobelEdgeOperation,
  PrewittEdgeOperation,
  LaplacianEdgeOperation, LaplacianOfGaussianEdgeOperation, ScaleNearestNeighborOperation,
} from "@/store";
import {Box, Button, Card, CardActions, CardContent, Grid, Paper, Typography} from "@mui/material";
import assert from "assert";
import {styled} from "@mui/system";

function PowerOperationCard(props: { operation: PowerOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Power</Typography>
      <Typography variant="h6">γ: {props.operation.gamma}</Typography>
    </CardContent>
  </Card>
}

function LinearOperationCard(props: { operation: LinearOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Linear</Typography>
      <Typography variant="h6">scale: {props.operation.gain}</Typography>
      <Typography variant="h6">bias: {props.operation.bias}</Typography>
    </CardContent>
  </Card>
}

function RotationOperationCard(props: { operation: RotationOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Rotation</Typography>
      <Typography variant="h6">Angle: {props.operation.angle}</Typography>
    </CardContent>
  </Card>
}

function AddSaltOperationCard(props: { operation: AddSaltOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Add Salt</Typography>
      <Typography variant="h6">probability: {props.operation.probability}</Typography>
    </CardContent>
  </Card>
}

function AddPepperOperationCard(props: { operation: AddPepperOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Add Pepper</Typography>
      <Typography variant="h6">probability: {props.operation.probability} </Typography>
    </CardContent>
  </Card>
}

function ConvolutionOperationCard(props: { operation: ConvolutionOperation }) {
  const {kernel, width} = props.operation;


  const Item = styled(Paper)(({theme}) => ({
    padding: theme.spacing(2),
    textAlign: 'center',
    color: theme.palette.text.secondary,
  }));


  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Convolution</Typography>
      <Grid
          justifyContent="space-between"
          container
          spacing={1}>
        {
          kernel.map((cell, index) =>
              <Grid item key={index} xs={12 / width}>
                <Item>{cell}</Item>
              </Grid>
          )
        }
      </Grid>
    </CardContent>
  </Card>
}

function EqualizeOperationCard(props: { operation: EqualizeOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Equalize</Typography>
    </CardContent>
  </Card>
}

function MinFilterOperationCard(props: { operation: MinFilterOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Min Filter</Typography>
      <Typography variant="h6">distance: {props.operation.distance}</Typography>
    </CardContent>
  </Card>
}

function MaxFilterOperationCard(props: { operation: MaxFilterOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Max Filter</Typography>
      <Typography variant="h6">distance: {props.operation.distance}</Typography>
    </CardContent>
  </Card>
}


function MedianFilterOperationCard(props: { operation: MedianFilterOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Median Filter</Typography>
      <Typography variant="h6">distance: {props.operation.distance}</Typography>
    </CardContent>
  </Card>
}

function ShearOperationCard(props: { operation: ShearOpration }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Shear</Typography>
      <Typography variant="h6">λ: {props.operation.lambda}</Typography>
      <Typography variant="h6">μ: {props.operation.miu}</Typography>
    </CardContent>
  </Card>
}

function ScaleOperationCard(props: { operation: ScaleOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Scale</Typography>
      <Typography variant="h6">Width Factor: {props.operation.width_factor}</Typography>
      <Typography variant="h6">Height Factor: {props.operation.height_factor}
      </Typography>
    </CardContent>
  </Card>
}

function GreyScaleOperationCard(props: { operation: GreyScaleOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">GreyScale</Typography>
    </CardContent>
  </Card>
}

function CropRightOperationCard(props: { operation: CropRightOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Crop Right</Typography>
      <Typography variant="h6">removal: {props.operation.removal}</Typography>
    </CardContent>
  </Card>
}

function CropBottomOperationCard(props: { operation: CropBottomOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Crop Bottom</Typography>
      <Typography variant="h6">removal: {props.operation.removal}
      </Typography>
    </CardContent>
  </Card>
}

function FlipOperationCard(props: { operation: FlipOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Flip</Typography>
      <Typography variant="h6">axis: {props.operation.axis}</Typography>
    </CardContent>
  </Card>
}

function SobelEdgeOperationCard(props: { operation: SobelEdgeOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Sobel Edge</Typography>
      <Typography variant="h6">threshold: {props.operation.threshold}</Typography>
    </CardContent>
  </Card>
}

function PrewittEdgeOperationCard(props: { operation: PrewittEdgeOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Prewitt Edge</Typography>
      <Typography variant="h6">threshold: {props.operation.threshold}
      </Typography>
    </CardContent>
  </Card>
}

function LaplacianEdgeOperationCard(props: { operation: LaplacianEdgeOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Laplacian Edge</Typography>
      <Typography variant="h6">threshold: {props.operation.threshold}
      </Typography>
    </CardContent>
  </Card>
}

function LaplacianOfGaussianEdgeOperationCard(props: { operation: LaplacianOfGaussianEdgeOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Laplacian of Gaussian</Typography>
      <Typography variant="h6">threshold: {props.operation.threshold}
      </Typography>
    </CardContent>
  </Card>
}


function ScaleNearestNeighborOperationCard(props: { operation: ScaleNearestNeighborOperation }) {
  return <Card variant="outlined">
    <CardContent>
      <Typography variant="h5">Scale Nearest</Typography>
      <Typography variant="h6">Width Factor: {props.operation.width_factor}
      </Typography>
      <Typography variant="h6">Height Factor: {props.operation.height_factor}
      </Typography>
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
    case "Shear":
      return <ShearOperationCard operation={props.operation}/>
    case "Scale":
      return <ScaleOperationCard operation={props.operation}/>
    case "Equalize":
      return <EqualizeOperationCard operation={props.operation}/>
    case "MinFilter":
      return <MinFilterOperationCard operation={props.operation}/>
    case "MaxFilter":
      return <MaxFilterOperationCard operation={props.operation}/>
    case "MedianFilter":
      return <MedianFilterOperationCard operation={props.operation}/>
    case "GreyScale":
      return <GreyScaleOperationCard operation={props.operation}/>
    case "CropRight":
      return <CropRightOperationCard operation={props.operation}/>
    case "CropBottom":
      return <CropBottomOperationCard operation={props.operation}/>
    case "AddSalt":
      return <AddSaltOperationCard operation={props.operation}/>
    case "AddPepper":
      return <AddPepperOperationCard operation={props.operation}/>
    case "PrewittEdge":
      return <PrewittEdgeOperationCard operation={props.operation}/>
    case "SobelEdge":
      return <SobelEdgeOperationCard operation={props.operation}/>
    case "LaplacianEdge":
      return <LaplacianEdgeOperationCard operation={props.operation}/>
    case "LaplacianOfGaussianEdge":
      return <LaplacianOfGaussianEdgeOperationCard operation={props.operation}/>
    case "ScaleNearestNeighbor":
      return <ScaleNearestNeighborOperationCard operation={props.operation}/>
    default:
      return <Card><CardContent>Unknown Operation</CardContent></Card>
  }
}


export default function OperationsView(props: { operations: Operation[] }) {
  const dispatch = useAppDispatch();
  const popLast = () => {
    dispatch({type: "app/removeOperation"});
    dispatch({type: "app/runPipeline"});
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
          alignItems="stretch"
          spacing={1}>
        {props.operations.map((operation, index) =>
            <Grid item key={index}>
              <OperationCard operation={operation}/>
            </Grid>)}
      </Grid>
    </CardContent>

  </Card>
}