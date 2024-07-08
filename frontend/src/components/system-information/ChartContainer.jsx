// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import { Typography } from "@mui/material";

// Corrected to accept props in a single object
export default function ChartContainer({ chartName, children }) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column", 
        alignItems: "center",
        justifyContent: "center", 
        width: "800px",
        height: "100%", 
      }}
    >
      <Typography variant="h6" style={{ alignSelf: "flex-start" }}>
        {chartName}
      </Typography>
      {children}
    </div>
  );
}
