import {Button, Card, CardActions, CardContent, Typography} from "@mui/material";
import {useAppDispatch} from "@/store";

export default function HistogramForm() {
  const dispatch = useAppDispatch();
  const onClick = () => {
    dispatch({type: "app/addEqualizeOperation"});
    dispatch({type: "app/runPipeline"});
  }

  return <Card>
    <CardActions>
      <Button variant="outlined" onClick={onClick}>Add Equalize To Pipeline</Button>
    </CardActions>
  </Card>
}