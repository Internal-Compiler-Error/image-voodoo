import {configureStore, ThunkAction, Action, PayloadAction, createSlice} from '@reduxjs/toolkit'
import {TypedUseSelectorHook, useDispatch, useSelector} from "react-redux";

import {linear_transformation} from "wasm-image-voodoo";

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

export type RotationOperation = {
  variant: "Rotation";
  angle: number;
}

export type FlipOperation = {
  variant: "Flip";
  axis: "horizontal" | "vertical";
}

export type Operation = LinearOperation | PowerOperation | ConvolutionOperation | RotationOperation | FlipOperation;

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
    },
    addConvolutionOperation: (state, action: PayloadAction<Omit<ConvolutionOperation, "variant">>) => {
      state.operations.push({variant: "Convolution", ...action.payload});
    },
    addRotationOperation: (state, action: PayloadAction<Omit<RotationOperation, "variant">>) => {
      state.operations.push({variant: "Rotation", ...action.payload});
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
