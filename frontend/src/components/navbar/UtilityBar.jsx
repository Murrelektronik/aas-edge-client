// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import styled from "@emotion/styled";
import NotificationsIcon from "@mui/icons-material/Notifications";
import SettingsIcon from "@mui/icons-material/Settings";
import HelpOutlineIcon from "@mui/icons-material/HelpOutline";

const StyledUtilityBar = styled.div`
  display: flex;
  align-items: center;
  justify-content: space-around;
  width: 173px; // Removed quotes to apply CSS correctly
  height: 112px;
`;

export default function UtilityBar() {
  return (
    <StyledUtilityBar>
      <NotificationsIcon />
      <SettingsIcon />
      <HelpOutlineIcon />
    </StyledUtilityBar>
  );
}
