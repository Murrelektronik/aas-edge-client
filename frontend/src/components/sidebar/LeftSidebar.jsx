// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import { Box } from "@mui/material";
import SidebarItem from "./SidebarItem";
import { useSelector } from "react-redux";
import { camelCaseToSpaces, nameToPath } from "../../utils";
import { Link } from "react-router-dom";

function LeftSidebar({ isSidebarOpen = true }) {
  const { entities } = useSelector((state) => state.submodels);

  const availableLeftSidebarItemsName = [
    "Network Configuration",
    "System Information",
  ];

  let sidebarItems = [{ name: "Home", path: "/" }];
  // Transforming item names to paths
  if (entities && entities.links) {
    let addedSidebarItems = entities.links
      .map((link) => {
        const name = camelCaseToSpaces(link.rel); // Convert camel case to spaces
        const path = nameToPath(link.href); // Create a path from the link
        return { name, path };
      })
      .filter((item) => availableLeftSidebarItemsName.includes(item.name)); // Filter items by name

    // Correctly concatenate arrays to avoid nested arrays
    sidebarItems = [...sidebarItems, ...addedSidebarItems];
  }

  return (
    <Box
      sx={{
        width: !isSidebarOpen ? "0" : "293px",
        flexShrink: 0,
        position: "relative",
        top: 0,
        boxSizing: "border-box",
        backgroundColor: "white",
        color: "black",
        zIndex: 1200,
        transition: "all 0.1s ease-in-out",
        visibility: !isSidebarOpen ? "hidden" : "visible",
        borderLeft: "0",
        borderTop: "0",
        borderRight: "0.5px solid #333333",
        borderBottom: "0",
        borderStyle: "solid",
        borderColor: "#333333",
        height: "auto",
      }}
    >
      {sidebarItems.map((item, index) => (
        <SidebarItem key={index} sidebarItem={item.name} to={item.path} />
      ))}

      <Box
        sx={{
          position: "absolute",
          bottom: "20px",
          left: 0,
          width: "100%", // Ensure the image container spans the entire sidebar width
          textAlign: "center", // Center the image if it's not full width
        }}
      >
        <Link to="https://lni40.de/">
          <img
            src="/lni-aas.png"
            alt="lni logo"
            style={{ width: "auto", height: "20px" }}
          />
        </Link>
      </Box>
    </Box>
  );
}

export default LeftSidebar;
