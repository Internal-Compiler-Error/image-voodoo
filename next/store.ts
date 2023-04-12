import {configureStore, createSlice, PayloadAction} from '@reduxjs/toolkit'
import {useDispatch} from "react-redux";

import {
  convolve,
  equalize,
  filter,
  flip_along_x_axis,
  flip_along_y_axis,
  gamma_transformation,
  init,
  Kernel,
  laplacian_edge,
  laplacian_of_gaussian_edge,
  linear_transformation,
  rotate,
  scale_via_bilinear,
  shear_wasm,
  greyscale,
  crop_right,
  crop_bottom,
  add_salt,
  add_pepper,
} from "wasm-image-voodoo";

init();

export type LinearOperation = {
  variant: "Linear";
  bias: number;
  gain: number;
};

export type ShearOpration = {
  variant: "Shear";
  lambda: number;
  miu: number;
}

export type PowerOperation = {
  variant: "Power";
  gamma: number;
};

export type ConvolutionOperation = {
  variant: "Convolution";
  kernel: number[];
  width: number;
  height: number;
}

export type ScaleOperation = {
  variant: "Scale";
  width_factor: number;
  height_factor: number;
}

export type RotationOperation = {
  variant: "Rotation";
  angle: number;
}

export type FlipOperation = {
  variant: "Flip";
  axis: "x" | "y";
}

export type EqualizeOperation = {
  variant: "Equalize";
}

export type LaplacianEdgeOperation = {
  variant: "LaplacianEdge";
  threshold: number;
}

export type LaplacianOfGaussianEdgeOperation = {
  variant: "LaplacianOfGaussianEdge";
  threshold: number;
}

export type MinFilterOperation = {
  variant: "MinFilter";
  distance: number;
}

export type MaxFilterOperation = {
  variant: "MaxFilter";
  distance: number;
}

export type MedianFilterOperation = {
  variant: "MedianFilter";
  distance: number;
}

export type GreyScaleOperation = {
  variant: "GreyScale";
}

export type CropRightOperation = {
  variant: "CropRight";
  removal: number;
}

export type CropBottomOperation = {
  variant: "CropBottom";
  removal: number;
}

export type AddSaltOperation = {
  variant: "AddSalt";
  probability: number;
}

export type AddPepperOperation = {
  variant: "AddPepper";
  probability: number;
}

export type Operation =
    LinearOperation
    | PowerOperation
    | ConvolutionOperation
    | RotationOperation
    | FlipOperation
    | EqualizeOperation
    | ScaleOperation
    | LaplacianEdgeOperation
    | ShearOpration
    | MinFilterOperation
    | MaxFilterOperation
    | MedianFilterOperation
    | LaplacianOfGaussianEdgeOperation
    | GreyScaleOperation
    | CropRightOperation
    | CropBottomOperation
    | AddSaltOperation
    | AddPepperOperation;


function evaluatePipeline(image: ImageData, operations: Operation[]) {
  return operations.reduce((image, operation) => {
    switch (operation.variant) {
      case "Linear":
        return linear_transformation(image, operation.gain, operation.bias);
      case "Power":
        return gamma_transformation(image, operation.gamma);
      case "Convolution":
        const arr = Float64Array.from(operation.kernel);
        const kernel = Kernel.from_vec(arr, operation.width, operation.height);
        return convolve(image, kernel, 0);
      case "Rotation":
        return rotate(image, operation.angle);
      case "Flip":
        if (operation.axis === "x") {
          return flip_along_x_axis(image);
        } else {
          return flip_along_y_axis(image);
        }
      case "Equalize":
        return equalize(image);
      case "Scale":
        return scale_via_bilinear(image, operation.width_factor, operation.height_factor);
      case "LaplacianEdge":
        return laplacian_edge(image, operation.threshold);
      case "LaplacianOfGaussianEdge":
        return laplacian_of_gaussian_edge(image, operation.threshold);
      case "Shear":
        return shear_wasm(image, operation.lambda, operation.miu);
      case "MinFilter":
        return filter(image, operation.distance, 0);
      case "MaxFilter":
        return filter(image, operation.distance, 1);
      case "MedianFilter":
        return filter(image, operation.distance, 2);
      case "GreyScale":
        return greyscale(image);
      case "CropRight":
        return crop_right(image, operation.removal);
      case "CropBottom":
        return crop_bottom(image, operation.removal);
      case "AddSalt":
        return add_salt(image, operation.probability);
      case "AddPepper":
        return add_pepper(image, operation.probability);
    }
  }, image);
}

export interface State {
  initial: ImageData | undefined;
  final: ImageData | undefined;
  operations: Operation[];
}


const initialState: State = {
  initial: undefined,
  final: undefined,
  operations: [],
};

const appSlice = createSlice({
  name: "app",
  initialState,
  reducers: {
    setInitial: (state, action: PayloadAction<ImageData>) => {
      state.initial = action.payload;
      // we only do this so the image viewer have something to show
      state.final = action.payload;
    },
    addLinearOperation: (state, action: PayloadAction<Omit<LinearOperation, "variant">>) => {
      state.operations.push({variant: "Linear", ...action.payload});
    },
    addPowerOperation: (state, action: PayloadAction<Omit<PowerOperation, "variant">>) => {
      state.operations.push({variant: "Power", ...action.payload});
    },
    addScaleOperation: (state, action: PayloadAction<Omit<ScaleOperation, "variant">>) => {
      state.operations.push({variant: "Scale", ...action.payload});
    },
    addConvolutionOperation: (state, action: PayloadAction<Omit<ConvolutionOperation, "variant">>) => {
      state.operations.push({variant: "Convolution", ...action.payload});
    },
    addRotationOperation: (state, action: PayloadAction<Omit<RotationOperation, "variant">>) => {
      state.operations.push({variant: "Rotation", ...action.payload});
    },
    addEqualizeOperation: (state, action: PayloadAction<Omit<EqualizeOperation, "variant">>) => {
      state.operations.push({variant: "Equalize", ...action.payload});
    },
    addLaplacianEdgeOperation: (state, action: PayloadAction<Omit<LaplacianEdgeOperation, "variant">>) => {
      state.operations.push({variant: "LaplacianEdge", ...action.payload});
    },
    addLaplacianOfGaussianEdgeOperation: (state, action: PayloadAction<Omit<LaplacianOfGaussianEdgeOperation, "variant">>) => {
      state.operations.push({variant: "LaplacianOfGaussianEdge", ...action.payload});
    },
    addShearOperation: (state, action: PayloadAction<Omit<ShearOpration, "variant">>) => {
      state.operations.push({variant: "Shear", ...action.payload});
    },
    addFlipOperation: (state, action: PayloadAction<Omit<FlipOperation, "variant">>) => {
      state.operations.push({variant: "Flip", ...action.payload});
    },
    addMinFilterOperation: (state, action: PayloadAction<Omit<MinFilterOperation, "variant">>) => {
      state.operations.push({variant: "MinFilter", ...action.payload});
    },
    addMaxFilterOperation: (state, action: PayloadAction<Omit<MaxFilterOperation, "variant">>) => {
      state.operations.push({variant: "MaxFilter", ...action.payload});
    },
    addMedianFilterOperation: (state, action: PayloadAction<Omit<MedianFilterOperation, "variant">>) => {
      state.operations.push({variant: "MedianFilter", ...action.payload});
    },
    addGreyScaleOperation: (state) => {
      state.operations.push({variant: "GreyScale"});
    },
    removeOperation: (state) => {
      state.operations.pop();
    },
    addCropRightOperation: (state, action: PayloadAction<Omit<CropRightOperation, "variant">>) => {
      state.operations.push({variant: "CropRight", ...action.payload});
    },
    addCropBottomOperation: (state, action: PayloadAction<Omit<CropBottomOperation, "variant">>) => {
      state.operations.push({variant: "CropBottom", ...action.payload});
    },
    addAddSaltOperation: (state, action: PayloadAction<Omit<AddSaltOperation, "variant">>) => {
      state.operations.push({variant: "AddSalt", ...action.payload});
    },
    addAddPepperOperation: (state, action: PayloadAction<Omit<AddPepperOperation, "variant">>) => {
      state.operations.push({variant: "AddPepper", ...action.payload});
    },
    runPipeline: (state) => {
      if (state.operations.length === 0 || !state.initial) {
        return;
      } else {
        const initial = state.initial;
        const operations = state.operations;
        state.final = evaluatePipeline(initial, operations);
      }
    }
  },
});


const reducer = appSlice.reducer;

export function makeStore() {
  return configureStore({
    preloadedState: initialState,
    reducer: reducer,
  });
}

const store = makeStore()

export type AppState = ReturnType<typeof store.getState>

export type AppDispatch = typeof store.dispatch


export const useAppDispatch = () => useDispatch<AppDispatch>()
export default store;
