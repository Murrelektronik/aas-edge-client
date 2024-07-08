// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React, { useState, useEffect } from "react";
import { pictureService } from "../../services/pictureService";

function ImageFromBackend() {
  const [imageUrl, setImageUrl] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchImage = async () => {
      try {
        const response = await pictureService.getPicture();
        const url = URL.createObjectURL(response.data);
        setImageUrl(url);
        setLoading(false);
      } catch (err) {
        setError("Failed to load image: " + err.message);
        setLoading(false);
      }
    };
    fetchImage();
  }, []); // Empty dependency array means this effect runs only once after the initial render

  if (loading) return <p>Loading image...</p>;
  if (error) return <p>Error loading image: {error}</p>;

  return (
    <img
      src={imageUrl}
      alt="Loaded from backend"
      style={{ maxWidth: "100%", height: "auto" }}
    />
  );
}

export default ImageFromBackend;
