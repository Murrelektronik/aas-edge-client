// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import NetworkInterfaces from "../components/network-configuration/NetworkInterfaces";
import PageContainer from "./PageContainer";

export default function NetworkConfigurationPage() {
  return (
    <PageContainer name="Network Configuration">
      <NetworkInterfaces />
    </PageContainer>
  );
}
