import { combineReducers, configureStore } from '@reduxjs/toolkit';
import imagesReducer from './slices/imagesSlice';
import submodelsReducer from './slices/submodelsSlice';

const rootReducer = combineReducers({
    images: imagesReducer,
    submodels: submodelsReducer,
});

const store = configureStore({
    reducer: rootReducer,
});

export default store;
