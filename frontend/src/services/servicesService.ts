import { api } from '../utils/apiClient';

export interface Service {
  name: string;
  available: boolean;
}

export type ServicesResponse = Array<Service>

/**
 * Service for managing API services
 */
export const servicesService = {
  /**
   * Get list of available services
   */
  async getServices(): Promise<Service[]> {
    try {
      const response = await api.get<ServicesResponse>('/services');
      return response || [];
    } catch (error) {
      console.error('Failed to fetch services:', error);
      throw new Error('Failed to fetch services');
    }
  },

  /**
   * Get service details by name
   */
  async getServiceDetails(serviceName: string): Promise<Service> {
    try {
      const response = await api.get<Service>(`/services/${serviceName}`);
      return response;
    } catch (error) {
      console.error(`Failed to fetch service details for ${serviceName}:`, error);
      throw new Error(`Failed to fetch service details for ${serviceName}`);
    }
  },
};
