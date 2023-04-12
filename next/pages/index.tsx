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
  ThemeProvider, createTheme, Box, Grid, Paper
} from "@mui/material";
import OperationsView from "@/componenets/operations_view";
import RotationForm from "@/componenets/rotation_form";
import PowerForm from "@/componenets/power_form";
import HistogramForm from "@/componenets/histogram_form";
import GPT4 from "@/componenets/gpt4_magic";
import FilteringForm from "@/componenets/filtering_form";
import EdgeForm from "@/componenets/edge_form";
import {green, pink, purple} from "@mui/material/colors";
import ImageViewer from "@/pages/image_viewer";
import ImageUploader from "@/componenets/image_uploader";
import ShearForm from "@/componenets/shear_form";
import ScaleForm from "@/componenets/scaling_form";
import FlipForm from "@/componenets/flip_form";
import Misc from "@/componenets/misc";
import Crop from "@/componenets/crop";
import Image from "next/image";


const LiveView = connect((state: State) => {
  return {
    operations: state.operations
  }
})(OperationsView);


const primaryGreen = green[500];
const accentGreen = green.A200;
const darkGreen = green[900];
const primaryPink = pink[500];
const accentPink = pink.A200;
const darkPink = pink[900];

const darkTheme = createTheme({
  palette: {
    primary: {
      light: accentPink,
      main: primaryPink,
      dark: darkPink,
      contrastText: "#fff"
    },
    mode: "dark",
    secondary: {
      light: accentGreen,
      main: primaryGreen,
      dark: darkGreen,
      contrastText: "#fff"
    }
  }
})

export default function Home() {
  return <ThemeProvider theme={darkTheme}>
    <img id="img-meme" src="https://s3.fission.codes/2022/10/rust_poster.png" hidden alt=""></img>

    <Paper
        variant="outlined"
        style={{
          position: "fixed",
          top: 0,
          left: 0,
          bottom: 0,
          right: 0,
          overflow: "auto",
        }}>
      <Container maxWidth="xl">
        <Provider store={store}>
          <Typography variant="h1" align="center">Image Voodoo</Typography>

          <Typography variant="h6">You maybe have heard of ImageMagick, now get ready for Image Voodoo. It does the
            same thing, but it&apos;s worse.
          </Typography>
          <Divider/>


          <Grid
              container
              direction="row"
              justifyContent="space-between"
              spacing={2}
              alignItems="stretch">
            <Grid item xs={6}>
              <ImageUploader></ImageUploader>
            </Grid>

            <Grid item xs={6}>
              <Box>
                <Accordion>
                  <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                    <Typography variant="h4">Misc</Typography>
                  </AccordionSummary>
                  <AccordionDetails><Misc/></AccordionDetails>
                </Accordion>

                <Accordion>
                  <AccordionSummary expandIcon={<ExpandMoreIcon/>}>
                    <Typography variant="h4">Crop</Typography>
                  </AccordionSummary>
                  <AccordionDetails><Crop/></AccordionDetails>
                </Accordion>


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

              </Box>
            </Grid>

            <Grid item xs={12}>
              <LiveView/>
            </Grid>


          </Grid>


        </Provider>
      </Container>
    </Paper>
  </ThemeProvider>
}
