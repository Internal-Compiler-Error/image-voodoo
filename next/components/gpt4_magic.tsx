import {Button, Card, CardActions, CardContent, Paper, Typography} from "@mui/material";

export default function GPT4() {
  return <Card>
    <CardContent>
      <Paper elevation={3}>
        <Typography variant="body1" paragraph>
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent et finibus urna. Curabitur felis augue,
          maximus eget rhoncus vel, bibendum et ante. In vel ultrices lorem. Nullam porta molestie erat id placerat.
          Suspendisse varius, enim eu mattis tristique, dolor risus semper ante, at ultrices nisl nulla in lectus.
          Vivamus interdum facilisis urna at congue. Duis suscipit molestie nisl, eu maximus nisi. Maecenas eget
          vulputate velit. Aenean arcu nisi, sollicitudin quis tortor sit amet, faucibus mollis est. Nam neque ipsum,
          tincidunt a tempus quis, sagittis sit amet lorem. Etiam ut eros eget velit auctor hendrerit vitae non lacus.
          Sed at suscipit eros, vitae vulputate turpis.
        </Typography>

        <Typography variant="body1" paragraph>
          Proin molestie tincidunt consequat. Quisque et elit rhoncus, venenatis orci nec, commodo purus. Praesent ex
          sem, lobortis ac lacinia in, tincidunt id dolor. Curabitur mattis eu quam eu gravida. Curabitur justo turpis,
          eleifend in neque non, auctor varius urna. Aliquam consequat, libero non condimentum hendrerit, nulla eros
          placerat erat, non interdum sem ipsum eu est. Cras pretium in urna vitae volutpat. Quisque odio nulla,
          vulputate sit amet elit sed, gravida finibus est. Sed ut condimentum sem. Nam ullamcorper scelerisque tellus,
          at efficitur turpis pharetra vel. Fusce consectetur luctus nulla ut feugiat. Quisque finibus viverra blandit.
        </Typography>
      </Paper>
    </CardContent>

    <CardActions>
      <Button variant="outlined">Add To Pipeline</Button>
    </CardActions>

  </Card>
}