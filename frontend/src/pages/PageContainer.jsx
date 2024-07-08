// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import styled from "@emotion/styled";

import { Typography } from "@mui/material";
import { useTheme } from "@emotion/react";

const StyledPageContainer = styled.div`
  display: flex;
  flex-direction: column;
  padding-top: 33px;
  margin-bottom: 33px;
`;

const StyledPageContentContainer = styled.div`
  display: flex;
  width: 100%;
  flex-direction: column;
  align-items: center;
  text-align: center;
`;

export default function PageContainer({ name, children }) {
  const customTheme = useTheme();
  return (
    <StyledPageContainer>
      <Typography
        variant="h5"
        style={{
          color: customTheme.mainColor,
          position: "relative",
          width: "400px",
          height: "30px",
          gap: "0px",
          marginBottom: "25px",
          opacity: "0px",
          textAlign: "start",
        }}
      >
        {name}
      </Typography>
      <StyledPageContentContainer>{children}</StyledPageContentContainer>
    </StyledPageContainer>
  );
}
