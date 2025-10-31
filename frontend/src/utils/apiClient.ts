/**
 * API Client Utility
 * Centralized API client with consistent error handling and configuration
 */

import { API_CONFIG, getApiUrl } from '../config/api';

// Custom error class for API errors
export class ApiError extends Error {
  public status: number;
  public response?: any;
  
  constructor(
    message: string,
    status: number,
    response?: any
  ) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
    this.response = response;
  }
}

// Request options interface
interface RequestOptions extends RequestInit {
  timeout?: number;
}

/**
 * Enhanced fetch with timeout and error handling
 */
const fetchWithTimeout = async (
  url: string,
  options: RequestOptions = {}
): Promise<Response> => {
  const { timeout = API_CONFIG.TIMEOUT, ...fetchOptions } = options;

  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

  try {
    const response = await fetch(url, {
      ...fetchOptions,
      signal: controller.signal,
    });

    clearTimeout(timeoutId);
    return response;
  } catch (error) {
    clearTimeout(timeoutId);
    if (error instanceof Error && error.name === 'AbortError') {
      throw new ApiError('Request timeout', 408);
    }
    throw error;
  }
};

/**
 * API Client class with common methods
 */
export class ApiClient {
  private baseHeaders: HeadersInit;

  constructor() {
    this.baseHeaders = {
      ...API_CONFIG.DEFAULT_HEADERS,
    };
  }

  /**
   * Set authorization header
   */
  setAuthToken(token: string): void {
    this.baseHeaders = {
      ...this.baseHeaders,
      'Authorization': `Bearer ${token}`,
    };
  }

  /**
   * Remove authorization header
   */
  clearAuthToken(): void {
    const { Authorization, ...headers } = this.baseHeaders as any;
    this.baseHeaders = headers;
  }

  /**
   * Make a GET request
   */
  async get<T = any>(endpoint: string, options: RequestOptions = {}): Promise<T> {
    return this.request<T>(endpoint, {
      ...options,
      method: 'GET',
    });
  }

  /**
   * Make a POST request
   */
  async post<T = any>(endpoint: string, data?: any, options: RequestOptions = {}): Promise<T> {
    return this.request<T>(endpoint, {
      ...options,
      method: 'POST',
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  /**
   * Make a PUT request
   */
  async put<T = any>(endpoint: string, data?: any, options: RequestOptions = {}): Promise<T> {
    return this.request<T>(endpoint, {
      ...options,
      method: 'PUT',
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  /**
   * Make a DELETE request
   */
  async delete<T = any>(endpoint: string, options: RequestOptions = {}): Promise<T> {
    return this.request<T>(endpoint, {
      ...options,
      method: 'DELETE',
    });
  }

  /**
   * Generic request method
   */
  private async request<T = any>(
    endpoint: string,
    options: RequestOptions = {}
  ): Promise<T> {
    const url = getApiUrl(endpoint);

    const requestOptions: RequestInit = {
      ...options,
      headers: {
        ...this.baseHeaders,
        ...options.headers,
      },
    };

    try {
      const response = await fetchWithTimeout(url, requestOptions);

      if (!response.ok) {
        let errorMessage = `HTTP ${response.status}: ${response.statusText}`;
        let errorData;

        try {
          errorData = await response.json();
          errorMessage = errorData.message || errorMessage;
        } catch {
          // If response is not JSON, use status text
        }

        throw new ApiError(errorMessage, response.status, errorData);
      }

      // Handle empty responses
      const contentType = response.headers.get('content-type');
      if (contentType && contentType.includes('application/json')) {
        return await response.json();
      }

      return await response.text() as T;
    } catch (error) {
      if (error instanceof ApiError) {
        throw error;
      }

      // Network or other errors
      throw new ApiError(
        error instanceof Error ? error.message : 'Unknown error occurred',
        0
      );
    }
  }
}

// Create and export a default instance
export const apiClient = new ApiClient();

// Export convenience functions
export const api = {
  get: <T = any>(endpoint: string, options?: RequestOptions) =>
    apiClient.get<T>(endpoint, options),

  post: <T = any>(endpoint: string, data?: any, options?: RequestOptions) =>
    apiClient.post<T>(endpoint, data, options),

  put: <T = any>(endpoint: string, data?: any, options?: RequestOptions) =>
    apiClient.put<T>(endpoint, data, options),

  delete: <T = any>(endpoint: string, options?: RequestOptions) =>
    apiClient.delete<T>(endpoint, options),

  setAuthToken: (token: string) => apiClient.setAuthToken(token),
  clearAuthToken: () => apiClient.clearAuthToken(),
};
