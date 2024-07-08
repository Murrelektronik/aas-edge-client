// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import React, { useState, useEffect } from "react";
import { submodelsService } from "../../services/submodelsService";
import styled from '@emotion/styled';

const StyledNetworkInterfaceInput = styled.input`
  width: 60%;
  padding: 5px;
`;

const updateSubmodel = async (submodelName, data) => {
  // Implement the API call to update the submodel on the backend
  return submodelsService.updateSubmodel(submodelName, data);
};

const StyledNetworkInterfaces = styled.div`
  width: 624px;
  height: auto;
  display: flex;
  flex-direction: column;
`;

const StyledNetworkInterfaceContent = styled.div`
  padding: 20px 10px 0 0;
`;

const StyledNetworkInterfaceElement = styled.div`
  height: 32px;
  display: flex;
  justify-content: space-between;
  padding: 5px;
  border-bottom: 1px solid #ccc; // Add a bottom border for visual separation
`;

export default function NetworkInterfaces() {
  const [networkConfigurations, setNetworkConfigurations] = useState(null);
  const [editMode, setEditMode] = useState(false);
  const [editedConfigs, setEditedConfigs] = useState(null);

  useEffect(() => {
    const fetchSubmodel = async () => {
      try {
        const response = await submodelsService.getSubmodel("NetworkConfiguration");
        setNetworkConfigurations(response.data.NetworkSetting);
        setEditedConfigs(response.data.NetworkSetting); // Initialize edit state with fetched data
      } catch (error) {
        console.error("Failed to fetch submodel:", error);
      }
    };

    fetchSubmodel();
  }, []);

  const handleEditToggle = () => {
    setEditMode(!editMode);
  };

  const handleChange = (iface, key, value) => {
    setEditedConfigs(prev => ({
      ...prev,
      [iface]: {
        ...prev[iface],
        [key]: value
      }
    }));
  };

  const handleSave = async () => {
    try {
      await updateSubmodel("NetworkConfiguration", editedConfigs);
      setNetworkConfigurations(editedConfigs);
      setEditMode(false);
    } catch (error) {
      console.error("Failed to update submodel:", error);
    }
  };

  return (
    <StyledNetworkInterfaces>
      <button onClick={handleEditToggle}>{editMode ? "Cancel" : "Edit"}</button>
      {editMode && <button onClick={handleSave}>Save</button>}
      {Object.entries(networkConfigurations || {}).map(([iface, config]) => (
        <StyledNetworkInterfaceContent key={iface}>
          <h3>{iface}</h3>
          {Object.entries(config).map(([key, value]) => (
            <StyledNetworkInterfaceElement key={key}>
              {editMode ? (
                <>
                  <span>{key}: </span>
                  <StyledNetworkInterfaceInput
                    type="text"
                    value={editedConfigs[iface][key]}
                    onChange={(e) => handleChange(iface, key, e.target.value)}
                  />
                </>
              ) : (
                <>
                  <span>{key}: </span>
                  <span>{value || 'N/A'}</span>
                </>
              )}
            </StyledNetworkInterfaceElement>
          ))}
        </StyledNetworkInterfaceContent>
      ))}
    </StyledNetworkInterfaces>
  );
}
