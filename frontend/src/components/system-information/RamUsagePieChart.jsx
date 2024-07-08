// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import PieChart from "../charts/PieChart";
import { Typography } from "@mui/material";
import ChartContainer from "./ChartContainer";

// Enhanced helper function to convert memory sizes to GiB
const convertToGiB = (value, unit) => {
  const unitConversion = {
    Mi: 1 / 1024,
    Mb: 1 / (1024 * 1.024), // Assuming Mb meant as Megabits
    Gi: 1,
    GB: 1 / 1.073741824, // 1 GiB = 1.073741824 GB
  };

  if (unitConversion[unit]) {
    return value * unitConversion[unit];
  } else {
    return 0;
  }
};

// Updated to extract the numeric value and unit from a memory size string, handling both GiB and GB
const parseMemorySize = (memorySize) => {
  // Remove spaces from the memorySize string to ensure correct matching
  const normalizedMemorySize = memorySize.replace(/\s+/g, "");
  const regex = /([\d.]+)(Mi|Gi|MB|GB)/;
  const match = normalizedMemorySize.match(regex);
  if (match) {
    return { value: parseFloat(match[1]), unit: match[2] };
  }
  return { value: 0, unit: "" };
};

const CustomTooltip = ({ active, payload }) => {
  if (active && payload && payload.length) {
    return (
      <div
        style={{
          backgroundColor: "#fff",
          padding: "5px",
          border: "1px solid #ccc",
        }}
      >
        <p>{`${payload[0].name}: ${payload[0].value}GB (${payload[0].payload.percentage}%)`}</p>
      </div>
    );
  }

  return null;
};

const RamUsagePieChart = ({ ramFree, ramInstalled }) => {
  // Parse the RAM sizes
  const { value: freeValue, unit: freeUnit } = parseMemorySize(ramFree);
  const { value: installedValue, unit: installedUnit } =
    parseMemorySize(ramInstalled);

  // Ensure both sizes are in GiB
  const ramFreeGi = convertToGiB(freeValue, freeUnit);
  const ramInstalledGi = convertToGiB(installedValue, installedUnit);

  // Calculate used RAM in GiB
  const ramUsedGi = ramInstalledGi - ramFreeGi;

  // Prepare the data for the pie chart
  const data = [
    {
      name: "Used RAM",
      value: parseFloat(ramUsedGi.toFixed(2)),
      percentage: ((ramUsedGi / ramInstalledGi) * 100).toFixed(2),
    },
    {
      name: "Free RAM",
      value: parseFloat(ramFreeGi.toFixed(2)),
      percentage: ((ramFreeGi / ramInstalledGi) * 100).toFixed(2),
    },
  ];

  return (
    <ChartContainer chartName="Ram Usage">
      <PieChart data={data} CustomTooltip={CustomTooltip} />
    </ChartContainer>
  );
};

export default RamUsagePieChart;
