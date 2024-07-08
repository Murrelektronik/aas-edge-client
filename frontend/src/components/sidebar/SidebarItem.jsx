// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import styled from "@emotion/styled";
import { Link } from "react-router-dom"; // Import Link from react-router-dom

const StyledSidebarItem = styled(Link)` // Use Link here instead of div
  display: flex;
  width: 293px;
  height: 48px;
  justify-content: start;
  align-items: center;
  padding: 8px 16px;
  text-decoration: none; // Remove underline from links
  color: inherit; // Use the inherited text color
  &:hover {
    background-color: #f0f0f0; // Optional: change background on hover
  }
`;

export default function SidebarItem({sidebarItem = "Home", to = "/"}) {
  return (
    <StyledSidebarItem to={to}>
      {sidebarItem}
    </StyledSidebarItem>
  )
}
