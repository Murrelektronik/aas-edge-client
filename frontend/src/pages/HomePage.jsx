// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import styled from "@emotion/styled";
import ProductDetailsTable from "../components/tables/ProductDetailsTable";
import ImageFromBackend from "../components/common/ImageFromBackend";

const StyledHomePage = styled.div`
  display: flex;
  width: 100%;
  max-width: 704px;
  flex-direction: column;
  align-items: center;
  text-align: center; // This ensures that the text content is also centered
  padding: 66px 0;
  gap: 20px;
`;


export default function HomePage() {
  return (
    <StyledHomePage>
      {/* Apply the CenteredImage styled component to your img */}
      <ImageFromBackend />
      <ProductDetailsTable />
    </StyledHomePage>
  );
}
