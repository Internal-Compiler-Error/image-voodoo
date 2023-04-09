import store, {State} from '@/store'
import {connect, Provider} from "react-redux";
import React from "react";

import LinearTransformationForm from "@/componenets/linear_transformation_form";
import ConvolutionForm from "@/componenets/convolution_form";
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import {
  Typography,
  Container,
  Divider,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  ThemeProvider, createTheme, Box, Grid
} from "@mui/material";
import OperationsView from "@/componenets/operations_view";
import RotationForm from "@/componenets/rotation_form";
import PowerForm from "@/componenets/power_form";
import HistogramForm from "@/componenets/histogram_form";
import GPT4 from "@/componenets/gpt4_magic";
import FilteringForm from "@/componenets/filtering_form";
import EdgeForm from "@/componenets/edge_form";
import {pink} from "@mui/material/colors";
import ImageViewer from "@/pages/image_viewer";
import ImageUploader from "@/componenets/image_uploader";
import ShearForm from "@/componenets/shear_form";
import ScaleForm from "@/componenets/scaling_form";
import FlipForm from "@/componenets/flip_form";


const LiveView = connect((state: State) => {
  return {
    operations: state.operations
  }
})(OperationsView);

const darkTheme = createTheme({
  palette: {
    primary: {
      main: pink[400],
    },
  }
})

export default function Home() {
  return <ThemeProvider theme={darkTheme}>
    {/*<Paper variant="outlined">*/}
    <Container>
      <Provider store={store}>
        <Typography variant="h1" align="center">Image Voodoo</Typography>

        <Typography variant="body1">You maybe have heard of ImageMagick, now get ready for Image Voodoo. It does the
          same
          {/* eslint-disable-next-line react/no-unescaped-entities */}
          thing, but it's worse.
        </Typography>
        <Divider/>


        <Grid container
              direction="row"
              justifyContent="space-between"
              spacing={2}
              alignItems="stretch">
          <Grid item xs>
            <ImageUploader></ImageUploader>
          </Grid>

          <Grid item xs>
            <Box>
              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Linear Transformation</Typography>
                </AccordionSummary>
                <AccordionDetails><LinearTransformationForm/></AccordionDetails>
              </Accordion>

              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Histogram Equalization</Typography>
                </AccordionSummary>
                <AccordionDetails><HistogramForm/></AccordionDetails>
              </Accordion>

              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Power Mapping</Typography>
                </AccordionSummary>
                <AccordionDetails><PowerForm/></AccordionDetails>
              </Accordion>

              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Convolution</Typography>
                </AccordionSummary>
                <AccordionDetails><ConvolutionForm/></AccordionDetails>
              </Accordion>

              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Rotation</Typography>
                </AccordionSummary>
                <AccordionDetails><RotationForm/></AccordionDetails>
              </Accordion>

              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Filtering</Typography>
                </AccordionSummary>
                <AccordionDetails><FilteringForm/></AccordionDetails>
              </Accordion>

              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Shear</Typography>
                </AccordionSummary>
                <AccordionDetails><ShearForm/></AccordionDetails>
              </Accordion>


              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Scale</Typography>
                </AccordionSummary>
                <AccordionDetails><ScaleForm/></AccordionDetails>
              </Accordion>

              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Flip</Typography>
                </AccordionSummary>
                <AccordionDetails><FlipForm/></AccordionDetails>
              </Accordion>

              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">Edge Detection</Typography>
                </AccordionSummary>
                <AccordionDetails><EdgeForm/></AccordionDetails>
              </Accordion>

              <Accordion>
                <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                  <Typography variant="h4">GPT-4</Typography>
                </AccordionSummary>
                <AccordionDetails><GPT4/></AccordionDetails>
              </Accordion>


            </Box>
          </Grid>

          <Grid item xs={12}>
            <LiveView/>
          </Grid>


        </Grid>


      </Provider>
    </Container>
    {/*</Paper>*/}
  </ThemeProvider>
}
