// Author: Pham-Minh-Khai Hoang (khai.hoang@yacoub.de)
import axios from 'axios';

export const ApiService = (customConfig) => {
  const instance = axios.create({
    // if relative path is used as base url, the host and port parts will be taken from browser's address bar.
    //baseURL: process.env.REACT_APP_API_BASE_URL || '/api',
    baseURL: process.env.NODE_ENV === 'development' ? 'http://localhost:18000/' : '/api',
    ...customConfig,
  });

  instance.interceptors.request.use(
    function (config) {
      // Do something before request is sent
      return config;
    },
    function (error) {
      // Do something with request error
      return Promise.reject(error);
    },
  );

  // Add a response interceptor
  instance.interceptors.response.use(
    function (response) {
      // Any status code that lie within the range of 2xx cause this function to trigger
      // Do something with response data
      return response;
    },
    function (error) {
      // Any status codes that falls outside the range of 2xx cause this function to trigger
      // Do something with response error
      return Promise.reject(error);
    },
  );

  return instance;
};
