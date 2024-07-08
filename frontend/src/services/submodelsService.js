// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import { ApiService } from './apiService';

export const submodelsService = {
  getSubmodels: async () => {
    return ApiService().get('');
  },
  getSubmodel: async (submodelName) => {
    // Use template literals to insert the variable part of the path
    return ApiService().get(`submodels/${submodelName}`);
  },
  updateSubmodel : async (submodelName, data) => {
    console.log("data send ", data);
    return ApiService().patch(`submodels/${submodelName}`, data);
  }
};