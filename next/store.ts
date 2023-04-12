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
    | LaplacianOfGaussianEdgeOperation;


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
      // state.image.push(action.payload);
      state.initial = action.payload;
      state.final = action.payload;
    },
    addLinearOperation: (state, action: PayloadAction<Omit<LinearOperation, "variant">>) => {
      state.operations.push({variant: "Linear", ...action.payload});

      // const [last] = state.image.slice(-1);
      //
      // const image = linear_transformation(last, action.payload.gain, action.payload.bias)
      // state.image.push(image)
    },
    addPowerOperation: (state, action: PayloadAction<Omit<PowerOperation, "variant">>) => {
      state.operations.push({variant: "Power", ...action.payload});


      // const [last] = state.image.slice(-1);
      //
      // const image = gamma_transformation(last, action.payload.gamma);
      // state.image.push(image);
    },
    addScaleOperation: (state, action: PayloadAction<Omit<ScaleOperation, "variant">>) => {
      state.operations.push({variant: "Scale", ...action.payload});

      // const [last] = state.image.slice(-1);
      // const {width_factor, height_factor} = action.payload;
      //
      // const image = scale_via_bilinear(last, width_factor, height_factor);
      // state.image.push(image);
    },
    addConvolutionOperation: (state, action: PayloadAction<Omit<ConvolutionOperation, "variant">>) => {
      state.operations.push({variant: "Convolution", ...action.payload});
      //
      // const [last] = state.image.slice(-1);
      // const arr = Float64Array.from(action.payload.kernel);
      // const kernel = Kernel.from_vec(arr, action.payload.width, action.payload.height);
      // const image = convolve(last, kernel, 0);
      // state.image.push(image);
    },
    addRotationOperation: (state, action: PayloadAction<Omit<RotationOperation, "variant">>) => {
      state.operations.push({variant: "Rotation", ...action.payload});

      // const [last] = state.image.slice(-1);
      // const image = rotate(last, action.payload.angle);
      // state.image.push(image);
    },
    addEqualizeOperation: (state, action: PayloadAction<Omit<EqualizeOperation, "variant">>) => {
      state.operations.push({variant: "Equalize", ...action.payload});

      // const [last] = state.image.slice(-1);
      // const image = equalize(last);
      // state.image.push(image);
    },
    addLaplacianEdgeOperation: (state, action: PayloadAction<Omit<LaplacianEdgeOperation, "variant">>) => {
      state.operations.push({variant: "LaplacianEdge", ...action.payload});

      // const [last] = state.image.slice(-1);
      // const {threshold} = action.payload;
      // const image = laplacian_edge(last, threshold);
      // state.image.push(image);
    },
    addLaplacianOfGaussianEdgeOperation: (state, action: PayloadAction<Omit<LaplacianOfGaussianEdgeOperation, "variant">>) => {
      // const [last] = state.image.slice(-1);
      // const {threshold} = action.payload;
      // const image = laplacian_of_gaussian_edge(last, threshold);
      // state.image.push(image);
      state.operations.push({variant: "LaplacianOfGaussianEdge", ...action.payload});
    },
    addShearOperation: (state, action: PayloadAction<Omit<ShearOpration, "variant">>) => {
      state.operations.push({variant: "Shear", ...action.payload});
      //
      // const [last] = state.image.slice(-1);
      // const {lambda, miu} = action.payload;
      // const image = shear_wasm(last, lambda, miu);
      // state.image.push(image);
    },
    addFlipOperation: (state, action: PayloadAction<Omit<FlipOperation, "variant">>) => {
      state.operations.push({variant: "Flip", ...action.payload});
      // const [last] = state.image.slice(-1);
      // const {axis} = action.payload;
      //
      // if (axis === "x") {
      //   const image = flip_along_x_axis(last);
      //   state.image.push(image);
      // } else {
      //   const image = flip_along_y_axis(last);
      //   state.image.push(image);
      // }
    },
    addMinFilterOperation: (state, action: PayloadAction<Omit<MinFilterOperation, "variant">>) => {
      state.operations.push({variant: "MinFilter", ...action.payload});
      // const [last] = state.image.slice(-1);
      // const {distance} = action.payload;
      // const image = filter(last, distance, 0);
      // state.image.push(image);
    },
    addMaxFilterOperation: (state, action: PayloadAction<Omit<MaxFilterOperation, "variant">>) => {
      state.operations.push({variant: "MaxFilter", ...action.payload});
      // const [last] = state.image.slice(-1);
      // const {distance} = action.payload;
      // const image = filter(last, distance, 1);
      // state.image.push(image);
    },
    addMedianFilterOperation: (state, action: PayloadAction<Omit<MedianFilterOperation, "variant">>) => {
      state.operations.push({variant: "MedianFilter", ...action.payload});
      // const [last] = state.image.slice(-1);
      // const {distance} = action.payload;
      // const image = filter(last, distance, 2);
      // state.image.push(image);
    },
    removeOperation: (state) => {
      state.operations.pop();
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


function ImageDataToDataUrl(imageData: ImageData) {
  const canvas = document.createElement("canvas");
  canvas.width = imageData.width;
  canvas.height = imageData.height;

  const ctx = canvas.getContext("2d");
  if (ctx) {
    ctx.putImageData(imageData, 0, 0);
    return canvas.toDataURL("image/png");
  } else {
    return "";
  }
}

function DataUrlToImageData(dataUrl: string) {
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d");
  if (ctx) {
    const img = new Image();
    img.src = dataUrl;
    ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
    return ctx.getImageData(0, 0, canvas.width, canvas.height);
  } else {
    return new ImageData(0, 0);
  }
}


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
