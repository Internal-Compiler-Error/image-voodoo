import {configureStore, ThunkAction, Action, PayloadAction, createSlice} from '@reduxjs/toolkit'
import {TypedUseSelectorHook, useDispatch, useSelector} from "react-redux";

import {
  linear_transformation,
  gamma_transformation,
  convolve,
  Kernel,
  rotate,
  equalize,
  scale_via_bilinear,
  init,
  laplacian_edge,
} from "wasm-image-voodoo";

init();

export type LinearOperation = {
  variant: "Linear";
  bias: number;
  gain: number;
};

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
  axis: "horizontal" | "vertical";
}

export type EqualizeOperation = {
  variant: "Equalize";
}

export type LaplacianEdgeOperation = {
  variant: "LaplacianEdge";
}

export type Operation =
    LinearOperation
    | PowerOperation
    | ConvolutionOperation
    | RotationOperation
    | FlipOperation
    | EqualizeOperation
    | ScaleOperation
    | LaplacianEdgeOperation;

export interface State {
  image: ImageData[];
  operations: Operation[];
}


const initialState: State = {
  image: [],
  operations: [],
};

const appSlice = createSlice({
  name: "app",
  initialState,
  reducers: {
    setImage: (state, action: PayloadAction<ImageData>) => {
      state.image.push(action.payload);
    },
    addLinearOperation: (state, action: PayloadAction<Omit<LinearOperation, "variant">>) => {
      state.operations.push({variant: "Linear", ...action.payload});

      const [last] = state.image.slice(-1);

      const image = linear_transformation(last, action.payload.gain, action.payload.bias)
      state.image.push(image)
    },
    addPowerOperation: (state, action: PayloadAction<Omit<PowerOperation, "variant">>) => {
      state.operations.push({variant: "Power", ...action.payload});


      const [last] = state.image.slice(-1);

      const image = gamma_transformation(last, action.payload.gamma);
      state.image.push(image);
    },

    addScaleOperation: (state, action: PayloadAction<Omit<ScaleOperation, "variant">>) => {
      state.operations.push({variant: "Scale", ...action.payload});

      const [last] = state.image.slice(-1);
      const {width_factor, height_factor} = action.payload;

      const image = scale_via_bilinear(last, width_factor, height_factor);
      state.image.push(image);
    },

    addConvolutionOperation: (state, action: PayloadAction<Omit<ConvolutionOperation, "variant">>) => {
      state.operations.push({variant: "Convolution", ...action.payload});

      const [last] = state.image.slice(-1);
      const arr = Float64Array.from(action.payload.kernel);
      const kernel = Kernel.from_vec(arr, action.payload.width, action.payload.height);
      const image = convolve(last, kernel, 0);
      state.image.push(image);
    },
    addRotationOperation: (state, action: PayloadAction<Omit<RotationOperation, "variant">>) => {
      state.operations.push({variant: "Rotation", ...action.payload});

      const [last] = state.image.slice(-1);
      const image = rotate(last, action.payload.angle);
      state.image.push(image);
    },
    addEqualizeOperation: (state, action: PayloadAction<Omit<EqualizeOperation, "variant">>) => {
      state.operations.push({variant: "Equalize", ...action.payload});

      const [last] = state.image.slice(-1);
      const image = equalize(last);
      state.image.push(image);
    },

    addLaplacianEdgeOperation: (state, action: PayloadAction<Omit<LaplacianEdgeOperation, "variant">>) => {
      state.operations.push({variant: "LaplacianEdge", ...action.payload});

      const [last] = state.image.slice(-1);
      const image = laplacian_edge(last);
      state.image.push(image);
    },

    addFlipOperation: (state, action: PayloadAction<Omit<FlipOperation, "variant">>) => {
      state.operations.push({variant: "Flip", ...action.payload});
    },
    removeOperation: (state) => {
      state.operations.pop();
    },
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
