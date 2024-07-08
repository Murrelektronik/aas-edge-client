import { createSlice } from "@reduxjs/toolkit";

const imagesSlice = createSlice({
  name: "images",
  initialState: {
    selectedImageFile: null,
  },
  reducers: {
    setSelectedImageFile: (state, action) => {
      state.selectedImageFile = action.payload;
    } 
  },
});

export const { setSelectedImageFile } = imagesSlice.actions;

export default imagesSlice.reducer;
