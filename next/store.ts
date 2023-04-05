import {configureStore, ThunkAction, Action, PayloadAction, createSlice} from '@reduxjs/toolkit'
import {TypedUseSelectorHook, useDispatch, useSelector} from "react-redux";

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
  kernel: number[][];
}

export type Operation = LinearOperation | PowerOperation | ConvolutionOperation;

export interface State {
  image: ImageData | undefined;
  operations: Operation[];
}


const initialState: State = {
  image: undefined,
  operations: [],
};

const appSlice = createSlice({
  name: "app",
  initialState,
  reducers: {
    setImage: (state, action: PayloadAction<ImageData>) => {
      state.image = action.payload;
    },
    addLinearOperation: (state, action: PayloadAction<Omit<LinearOperation, "variant">>) => {
      state.operations.push({variant: "Linear", ...action.payload});
    },

    addPowerOperation: (state, action: PayloadAction<Omit<PowerOperation, "variant">>) => {
      state.operations.push({variant: "Power", ...action.payload});
    },

    addConvolutionOperation: (state, action: PayloadAction<Omit<ConvolutionOperation, "variant">>) => {
      state.operations.push({variant: "Convolution", ...action.payload});
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

const IHateRedux = makeStore()

export type AppState = ReturnType<typeof IHateRedux.getState>

export type AppDispatch = typeof IHateRedux.dispatch


export const useAppDispatch = () => useDispatch<AppDispatch>()
export default IHateRedux;
