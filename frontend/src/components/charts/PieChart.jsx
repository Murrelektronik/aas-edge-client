// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from 'react';
import { PieChart as RePieChart, Pie, Cell, Tooltip, Legend } from 'recharts';
import { useTheme } from '@emotion/react';



const PieChart = ({ data, CustomTooltip }) => {
  const theme = useTheme();

  // Colors for each section
  const COLORS = [theme.mainColor, '#bdbdbd'];

  return (
    <RePieChart width={350} height={250}>
      <Pie
        data={data}
        cx="50%"
        cy="50%"
        innerRadius={80}
        outerRadius={100}
        fill="#8884d8"
        paddingAngle={5}
        dataKey="value"
      >
        {data.map((entry, index) => (
          <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
        ))}
      </Pie>
      <Tooltip content={<CustomTooltip />} />
      <Legend />
    </RePieChart>
  );
};

export default PieChart;
