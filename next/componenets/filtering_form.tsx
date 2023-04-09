import {Button, Card, CardActions, CardContent, Typography} from "@mui/material";

export default function FilteringForm() {
  return <Card>
    {/*<CardContent>*/}
    {/*  <Typography variant="body1">*/}
    {/*    Some default filters can be added directly, or you can do it manually using the convolution input form.*/}
    {/*  </Typography>*/}
    {/*</CardContent>*/}
    <CardActions>
      <Button variant={"outlined"}>Add Min Filter</Button>
      <Button variant={"outlined"}>Add Median Filter</Button>
      <Button variant={"outlined"}>Add Max Filter</Button>
      <Button variant={"outlined"}>Add 3x3 Gaussian</Button>
    </CardActions>
  </Card>
}