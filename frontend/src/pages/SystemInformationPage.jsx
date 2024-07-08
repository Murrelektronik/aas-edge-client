// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React, { useEffect, useState } from "react";
import { submodelsService } from "../services/submodelsService";

import PageContainer from "./PageContainer";
import RamUsagePieChart from "../components/system-information/RamUsagePieChart";
import CPUUsageTimeSeriesChart from "../components/system-information/CPUUsageTimeSeriesChart";
import BoardTemperaturesTimeSeriesChart from "../components/system-information/BoardTemperaturesTimeSeriesChart";

export default function SystemInformationPage() {
  const [systemInformation, setSystemInformation] = useState(null);
  const [cpuUsage, setCpuUsage] = useState([
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
  ]);
  const [boardTemperature, setBoardTemperature] = useState([
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
    null,
  ]);

  const extractCpuUsage = (newCpuUsageValue) => {
    setCpuUsage((oldCpuUsage) => {
      const numericValue = parseFloat(newCpuUsageValue.replace(/[^\d.-]/g, ''))
      // Create a copy of the array to manipulate
      const updatedCpuUsage = [...oldCpuUsage];
      updatedCpuUsage.push(numericValue);
      updatedCpuUsage.shift(); // Remove the first element
      return updatedCpuUsage;
    });
  };

  const extractBoardTemperature = (newBoardTemperature) => {
    setBoardTemperature((oldBoardTemperatures) => {
      const numericValue = parseFloat(newBoardTemperature.replace(/[^\d.-]/, ''))
      // Create a copy of the array to manipulate
      const updatedBoardTemperatures = [...oldBoardTemperatures];
      updatedBoardTemperatures.push(numericValue);
      updatedBoardTemperatures.shift(); // Remove the first element
      return updatedBoardTemperatures;
    });
  }

  useEffect(() => {
    const fetchSubmodel = async () => {
      try {
        const response = await submodelsService.getSubmodel(
          "SystemInformation"
        );
        setSystemInformation(response.data);
        extractCpuUsage(response.data.Hardware.Processor.CpuUsage);
        extractBoardTemperature(response.data.Hardware.BoardTemperature);
      } catch (error) {
        console.error("Failed to fetch submodel:", error);
      }
    };

    // Set up the interval
    const intervalId = setInterval(fetchSubmodel, 5000); 

    fetchSubmodel();

    return () => clearInterval(intervalId);
  }, []); // Fetch systeminformation from backend every 5 second

  return (
    <PageContainer name="System Information">
      {systemInformation && (
        <>
          <RamUsagePieChart
            ramFree={systemInformation.Hardware.Memory.RAMFree}
            ramInstalled={systemInformation.Hardware.Memory.RAMInstalled}
          />
          <CPUUsageTimeSeriesChart cpuUsage={cpuUsage}/>
          <BoardTemperaturesTimeSeriesChart boardTemperatures={boardTemperature} />
        </>
      )}
    </PageContainer>
  );
}
