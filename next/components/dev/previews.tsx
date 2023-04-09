import React from "react";
import {ComponentPreview, Previews} from "@react-buddy/ide-toolbox-next";
import {PaletteTree} from "./palette";
import ShearForm from "@/componenets/shear_form";

const ComponentPreviews = () => {
  return (
      <Previews palette={<PaletteTree/>}>
        <ComponentPreview path="/ShearForm">
          <ShearForm/>
        </ComponentPreview>
      </Previews>
  );
};

export default ComponentPreviews;