// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import { useEffect, useState } from "react";
import themeMUI from "../theme";

export default function useCustomTheme() {
  const [customTheme, setCustomTheme] = useState(themeMUI); // Initialize with themeMUI as the default

  useEffect(() => {
    const fetchTheme = async () => {
      try {
        const response = await fetch(`${process.env.PUBLIC_URL}/mount_volume/theme.json`);
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data = await response.json();
        // Assuming 'data' is an object that matches the structure needed to merge with themeMUI
        const mergedTheme = {
          ...themeMUI, // Spread the existing theme
          ...data, // Overwrite and add with custom theme data
        };
        setCustomTheme(mergedTheme);
      } catch (error) {
        console.error("Error loading theme", error);
      }
    };

    fetchTheme();
  }, []);

  return customTheme;
}
