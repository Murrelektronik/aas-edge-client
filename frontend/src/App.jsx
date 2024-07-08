// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React, { useEffect} from "react";
import { useDispatch } from 'react-redux';
import { ThemeProvider } from "@emotion/react";
import styled from "@emotion/styled";
import { CssBaseline } from "@mui/material";
import Navbar from "./components/navbar/Navbar";
import LeftSidebar from "./components/sidebar/LeftSidebar";
import { Routes, Route } from "react-router-dom";
import HomePage from "./pages/HomePage";
import { fetchSubmodels } from "./redux/slices/submodelsSlice";
import NetworkConfigurationPage from "./pages/NetworkConfigurationPage";
import SystemInformationPage from "./pages/SystemInformationPage";
import useCustomTheme from "./hooks/useCustomTheme";

const StyledAppContainer = styled.div`
  text-align: center;
  display: flex;
  flex-direction: column;
  min-height: 100vh; // Use the full viewport height
`;

const StyledMainContent = styled.div`
  display: flex;
  flex-grow: 1; // Allows content to fill the available space
  overflow: auto; // Adds scroll to the main content if necessary
`;

const StyledContentArea = styled.div`
  display: flex;
  flex-grow: 1;
  padding: 5px 5px;
  justify-content: center;
`;

function App() {
  const dispatch = useDispatch();
  const theme = useCustomTheme();

  useEffect(() => {
    dispatch(fetchSubmodels());
  }, [dispatch]);

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <StyledAppContainer>
        <Navbar />
        <StyledMainContent>
          <LeftSidebar />
          <StyledContentArea>
            <Routes>
              <Route path="/" element={<HomePage />} />
              <Route path="/submodels/network-configuration" element={<NetworkConfigurationPage />} />
              <Route path="/submodels/system-information" element={<SystemInformationPage />}/>
            </Routes>
          </StyledContentArea>
        </StyledMainContent>
      </StyledAppContainer>
    </ThemeProvider>
  );
}

export default App;
