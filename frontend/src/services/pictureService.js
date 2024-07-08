// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import { ApiService } from './apiService';

export const pictureService = {
  getPicture: async () => {
    return ApiService().get('/picture', { responseType: 'blob' });
  },
};