// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React, { useState, useEffect } from "react";
import styled from "@emotion/styled";
import { AppBar, Toolbar } from "@mui/material";
import UtilityBar from "./UtilityBar";
import UserProfileMenu from "./UserProfileMenu";
import SearchBar from "./Searchbar";
import { Link } from "react-router-dom";

const StyledImageContainer = styled.div`
  display: flex;
  width: 293px;
  height: 112px;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  border-right: 0.5px solid #333333; // Adds a right border
`;

export default function Navbar() {
  const [companyURL, setCompanyURL] = useState(null);

  // get company link from mount file
  useEffect(() => {
    // Define the async function inside the useEffect
    const fetchData = async () => {
      try {
        // Await the response of the fetch call
        const response = await fetch(`${process.env.PUBLIC_URL}/mount_volume/company-link.json`);
        if (!response.ok) {
          // Throw an error if the response is not ok
          throw new Error("Network response was not ok");
        }
        // Await the parsing of the JSON from the response
        const jsonData = await response.json();
        setCompanyURL(jsonData.companyURL);
      } catch (err) {
        console.log("error loading company url");
      }
    };

    // Call the fetchData function
    fetchData();
  }, []);

  return (
    <>
      <AppBar
        position="fixed"
        elevation={1}
        sx={{
          zIndex: (theme) => theme.zIndex.drawer + 1,
          backgroundColor: "white",
          color: "black",
          height: "112px",
          display: "flex",
          flexDirection: "row",
        }}
      >
        <StyledImageContainer>
          <Link to={companyURL}>
            <img
              src="/mount_volume/company-logo.png"
              alt="Logo"
              height="32px"
              style={{
                width: "auto",
                height: "42px",
              }}
            />
          </Link>
        </StyledImageContainer>
        <SearchBar />
        <UtilityBar />
        <UserProfileMenu />
      </AppBar>
      <Toolbar sx={{ height: "112px" }}>Toolbar</Toolbar>
    </>
  );
}
