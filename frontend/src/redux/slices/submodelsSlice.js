// features/submodels/submodelsSlice.js
import { createSlice, createAsyncThunk } from "@reduxjs/toolkit";
import { submodelsService } from "../../services/submodelsService";

const submodelsSlice = createSlice({
  name: "submodels",
  initialState: {
    entities: {},
    loading: "idle",
    error: null,
  },
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(fetchSubmodels.pending, (state) => {
        state.loading = "pending";
      })
      .addCase(fetchSubmodels.fulfilled, (state, action) => {
        state.loading = "succeeded";
        state.entities = action.payload;
      })
      .addCase(fetchSubmodels.rejected, (state, action) => {
        state.loading = "failed";
        state.error = action.error.message;
      });
  },
});

export default submodelsSlice.reducer;

export const fetchSubmodels = createAsyncThunk(
  "submodels/fetchSubmodels",
  async () => {
    const response = await submodelsService.getSubmodels();
    const data = await response.data;
    return data;
  }
);
