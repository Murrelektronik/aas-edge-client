// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React from "react";
import styled from "@emotion/styled";
import { Avatar, Menu, MenuItem, Button, Typography } from "@mui/material";
import ArrowDropDownIcon from "@mui/icons-material/ArrowDropDown";
import { useTheme } from "@emotion/react";

const StyledUserMenu = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  width: 296px;
  height: 112px;
  border-left: 0.5px solid #333333;
`;

const StyledNameAndEmail = styled.div`
  display: flex;
  flex-direction: column;
  margin-left: 8px;
  text-transform: none;
`;

export default function UserProfileMenu() {
  const [anchorEl, setAnchorEl] = React.useState(null);
  const open = Boolean(anchorEl);
  const theme = useTheme();

  const handleClick = (event) => {
    setAnchorEl(event.currentTarget);
  };

  const handleClose = () => {
    setAnchorEl(null);
  };

  // Dummy user data - replace with actual data from user's account
  const user = {
    name: "John Doe",
    email: "johndoe@example.com",
  };

  return (
    <StyledUserMenu>
      <Button
        id="user-profile-button"
        aria-controls={open ? "user-profile-menu" : undefined}
        aria-haspopup="true"
        aria-expanded={open ? "true" : undefined}
        onClick={handleClick}
        endIcon={<ArrowDropDownIcon />}
      >
        <Avatar src={user.avatarUrl} alt={user.name} />
        <StyledNameAndEmail>
          <Typography variant="body1" color={theme.mainColor}>{user.name}</Typography>
          <Typography variant="body2" color={theme.mainColor}>
            {user.email}
          </Typography>
        </StyledNameAndEmail>
      </Button>
      <Menu
        id="user-profile-menu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        MenuListProps={{
          "aria-labelledby": "user-profile-button",
        }}
      >
        <MenuItem onClick={handleClose}>Profile</MenuItem>
        <MenuItem onClick={handleClose}>Account</MenuItem>
        <MenuItem onClick={handleClose}>Logout</MenuItem>
      </Menu>
    </StyledUserMenu>
  );
}
