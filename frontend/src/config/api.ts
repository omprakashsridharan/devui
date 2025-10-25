/**
 * API Configuration
 * Central configuration for API base URLs based on environment
 */

// Environment detection
const isDevelopment = import.meta.env.DEV;
const isProduction = import.meta.env.PROD;

// API Base URL Configuration
export const API_CONFIG = {
  // Development: Use localhost:3000 with /dev/ui/api prefix
  // Production: Use /dev/ui/api prefix (relative URLs)
  BASE_URL: isDevelopment ? 'http://localhost:3000/dev/ui/api' : '/dev/ui/api',

  // API endpoints
  ENDPOINTS: {
    // Add your API endpoints here
    // Example:
    // USERS: '/users',
    // AUTH: '/auth',
    // DATA: '/data',
  },

  // Request configuration
  DEFAULT_HEADERS: {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
  },

  // Timeout configuration (in milliseconds)
  TIMEOUT: 10000,
} as const;

/**
 * Get the full API URL for an endpoint
 * @param endpoint - The API endpoint path
 * @returns Full URL combining base URL and endpoint
 */
export const getApiUrl = (endpoint: string): string => {
  // Remove leading slash if present to avoid double slashes
  const cleanEndpoint = endpoint.startsWith('/') ? endpoint.slice(1) : endpoint;

  // Combine base URL and endpoint
  const fullUrl = `${API_CONFIG.BASE_URL}/${cleanEndpoint}`;

  // Clean up any double slashes
  return fullUrl.replace(/\/+/g, '/').replace(':/', '://');
};

/**
 * Environment information for debugging
 */
export const ENV_INFO = {
  isDevelopment,
  isProduction,
  baseUrl: API_CONFIG.BASE_URL,
  mode: import.meta.env.MODE,
} as const;

// Log configuration in development
if (isDevelopment) {
  console.log('🔧 API Configuration:', {
    baseUrl: API_CONFIG.BASE_URL,
    environment: import.meta.env.MODE,
    endpoints: API_CONFIG.ENDPOINTS,
  });
}
