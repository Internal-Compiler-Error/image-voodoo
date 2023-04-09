import {Button, Card, CardActions} from "@mui/material";

export default function EdgeForm() {
  return <Card>
    {/*<CardContent>*/}
    {/*  <Typography variant="h4">Edge Detection</Typography>*/}
    {/*</CardContent>*/}

    <CardActions>
      <Button variant="outlined">Add Prewitt To Pipeline</Button>
      <Button variant="outlined">Add Sobel To Pipeline</Button>
      <Button variant="outlined">Add Laplacian To Pipeline</Button>
    </CardActions>

  </Card>
}