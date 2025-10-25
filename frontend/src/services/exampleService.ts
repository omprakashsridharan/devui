/**
 * Example Service
 * Demonstrates how to use the API client with the configured base URL
 */

import { api, ApiError } from '../utils/apiClient';

// Example API service functions
export const exampleService = {
  /**
   * Fetch data from the API
   */
  async fetchData(): Promise<Record<string, unknown>> {
    try {
      const data = await api.get('/data');
      return data;
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('API Error:', error.message, error.status);
        throw error;
      }
      throw new Error('Failed to fetch data');
    }
  },

  /**
   * Post data to the API
   */
  async postData(data: Record<string, unknown>): Promise<Record<string, unknown>> {
    try {
      const response = await api.post('/data', data);
      return response;
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('API Error:', error.message, error.status);
        throw error;
      }
      throw new Error('Failed to post data');
    }
  },

  /**
   * Update data via API
   */
  async updateData(id: string, data: Record<string, unknown>): Promise<Record<string, unknown>> {
    try {
      const response = await api.put(`/data/${id}`, data);
      return response;
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('API Error:', error.message, error.status);
        throw error;
      }
      throw new Error('Failed to update data');
    }
  },

  /**
   * Delete data via API
   */
  async deleteData(id: string): Promise<void> {
    try {
      await api.delete(`/data/${id}`);
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('API Error:', error.message, error.status);
        throw error;
      }
      throw new Error('Failed to delete data');
    }
  },
};
