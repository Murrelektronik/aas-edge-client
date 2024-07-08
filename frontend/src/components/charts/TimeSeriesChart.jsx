// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
} from "recharts";
import { useTheme } from "@emotion/react";

const TimeSeriesChart = ({
  data,
  formatXAxis,
  formatYAxis,
  xLabel,
  yLabel,
  yDomain,
  legendDescription,
  labelFormatter,
  valueFormatter,
}) => {
  const theme = useTheme();
  return (
    <LineChart
      width={800}
      height={320}
      data={data}
      margin={{
        top: 20,
        right: 30,
        left: 20,
        bottom: 5,
      }}
    >
      <CartesianGrid strokeDasharray="3 3" />
      <XAxis
        dataKey="time"
        name="Time"
        label={xLabel}
        tickFormatter={formatXAxis}
      />
      <YAxis label={yLabel} tickFormatter={formatYAxis} domain={yDomain} />
      <Tooltip formatter={valueFormatter} labelFormatter={labelFormatter} />
      <Legend />
      <Line
        type="monotone"
        dataKey="value"
        stroke={theme.mainColor}
        activeDot={{ r: 8 }}
        name={legendDescription}
        isAnimationActive={false}
      />
    </LineChart>
  );
};

export default TimeSeriesChart;
