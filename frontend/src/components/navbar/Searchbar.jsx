// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import styled from "@emotion/styled";
import { InputBase } from "@mui/material";
import SearchIcon from "@mui/icons-material/Search";

const SearchContainer = styled.div`
  display: flex;
  flex-grow: 1;
  border-right: 0.5px solid #333333;
`;

const StyledSearchBar = styled.div`
  display: flex;
  flex-grow: 1;

  margin: auto 16px; // Center the search bar with horizontal margin
  align-items: center;

  padding: 4px 16px; // Padding around the contents
`;

const StyledInputBase = styled(InputBase)`
  margin-left: 8px;
  flex-grow: 1;
`;

export default function SearchBar() {
  return (
    <SearchContainer>
      <StyledSearchBar>
        <SearchIcon color="action" />
        <StyledInputBase placeholder="Search…" />
      </StyledSearchBar>
    </SearchContainer>
  );
}
