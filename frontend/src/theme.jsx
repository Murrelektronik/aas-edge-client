import { createTheme } from "@mui/material/styles";
import { green as greenMUI, grey } from "@mui/material/colors";

export const mainColor = process.env.REACT_APP_MAIN_COLOR || "#37A40D";

const theme = createTheme({
	palette: {
		primary: {
			dark: greenMUI[900], // Darker shade of green
			main: greenMUI[800], // Main green color
			light: greenMUI[300], // Lighter shade of green
		},
		contrastThreshold: 4.5,
	},
	components: {
		background: {
			default: grey[300],
			user: greenMUI[200],
		},
		MuiButton: {
			styleOverrides: {
				root: {
					borderRadius: "25px",
				},
			},
		},
	},
	page: {
		background: "white",
		color: "black",
	},
});

export default theme;
