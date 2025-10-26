/**
 * SQL Service
 * Handles SQL connections and related operations
 */

import { api, ApiError } from '../utils/apiClient';

export interface SqlConnection {
  name: string;
  database_type: string;
  // Optional fields that might be present
  id?: string;
  host?: string;
  port?: number;
  database?: string;
  username?: string;
  status?: 'connected' | 'disconnected' | 'error';
  lastUsed?: string;
  description?: string;
}

// The API returns an array directly, not wrapped in an object
export type SqlConnectionsResponse = SqlConnection[];

/**
 * SQL Service class
 */
export class SqlService {
  /**
   * Get all SQL connections
   */
  async getConnections(): Promise<SqlConnection[]> {
    try {
      const response = await api.get<SqlConnectionsResponse>('/services/sql/connections');
      // The API returns an array directly, so we return it as is
      return response || [];
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('Failed to fetch SQL connections:', error.message);
        throw new Error(`Failed to fetch connections: ${error.message}`);
      }
      throw new Error('Failed to fetch SQL connections');
    }
  }

  /**
   * Get a specific connection by ID
   */
  async getConnection(id: string): Promise<SqlConnection> {
    try {
      const response = await api.get<SqlConnection>(`/services/sql/connections/${id}`);
      return response;
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('Failed to fetch SQL connection:', error.message);
        throw new Error(`Failed to fetch connection: ${error.message}`);
      }
      throw new Error('Failed to fetch SQL connection');
    }
  }

  /**
   * Test a connection
   */
  async testConnection(id: string): Promise<{ success: boolean; message: string }> {
    try {
      const response = await api.post<{ success: boolean; message: string }>(
        `/services/sql/connections/${id}/test`
      );
      return response;
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('Failed to test SQL connection:', error.message);
        throw new Error(`Failed to test connection: ${error.message}`);
      }
      throw new Error('Failed to test SQL connection');
    }
  }

  /**
   * Execute SQL query on a specific connection
   */
  async executeQuery(
    connectionId: string,
    query: string
  ): Promise<{
    success: boolean;
    data?: Record<string, unknown>[];
    columns?: string[];
    message?: string;
    executionTime?: number;
  }> {
    try {
      const response = await api.post<{
        success: boolean;
        data?: Record<string, unknown>[];
        columns?: string[];
        message?: string;
        executionTime?: number;
      }>(`/services/sql/connections/${connectionId}/execute`, {
        query,
      });
      return response;
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('Failed to execute SQL query:', error.message);
        throw new Error(`Failed to execute query: ${error.message}`);
      }
      throw new Error('Failed to execute SQL query');
    }
  }

  /**
   * Get database tables for a connection
   */
  async getTables(connectionId: string): Promise<Array<{
    name: string;
    schema: string;
    columns: Array<{
      name: string;
      data_type: string;
      is_nullable: boolean;
      is_primary_key: boolean;
      default_value: string | null;
    }>;
  }>> {
    try {
      const response = await api.get<Array<{
        name: string;
        schema: string;
        columns: Array<{
          name: string;
          data_type: string;
          is_nullable: boolean;
          is_primary_key: boolean;
          default_value: string | null;
        }>;
      }>>(`/services/sql/connections/${connectionId}/tables`);
      return response;
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('Failed to fetch tables:', error.message);
        throw new Error(`Failed to fetch tables: ${error.message}`);
      }
      throw new Error('Failed to fetch tables');
    }
  }

  /**
   * Get table data for a specific table with optional filters
   */
  async getTableData(
    connectionId: string,
    tableName: string,
    filters?: Record<string, string>
  ): Promise<{
    columns: string[];
    data: Record<string, unknown>[];
  }> {
    try {
      let url = `/services/sql/connections/${connectionId}/tables/${tableName}`;

      // Add query parameters if filters are provided
      if (filters && Object.keys(filters).length > 0) {
        const searchParams = new URLSearchParams();
        Object.entries(filters).forEach(([key, value]) => {
          if (value && value.trim() !== '') {
            searchParams.append(key, value.trim());
          }
        });

        if (searchParams.toString()) {
          url += `?${searchParams.toString()}`;
        }
      }

      const response = await api.get<Array<{
        columns: string[];
        data: Record<string, unknown>;
      }>>(url);

      // Transform the response to match expected format
      if (response.length > 0) {
        const firstItem = response[0];
        return {
          columns: firstItem.columns,
          data: response.map(item => item.data),
        };
      }

      return { columns: [], data: [] };
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('Failed to fetch table data:', error.message);
        throw new Error(`Failed to fetch table data: ${error.message}`);
      }
      throw new Error('Failed to fetch table data');
    }
  }

  /**
   * Get database schema for a connection (legacy method for compatibility)
   */
  async getSchema(connectionId: string): Promise<{
    tables: Array<{ name: string; type: string; schema?: string }>;
    views: Array<{ name: string; schema?: string }>;
  }> {
    try {
      const tables = await this.getTables(connectionId);
      return {
        tables: tables.map(table => ({
          name: table.name,
          type: 'table',
          schema: table.schema,
        })),
        views: [], // Views not provided in current API
      };
    } catch (error) {
      if (error instanceof ApiError) {
        console.error('Failed to fetch schema:', error.message);
        throw new Error(`Failed to fetch schema: ${error.message}`);
      }
      throw new Error('Failed to fetch schema');
    }
  }
}

// Export a default instance
export const sqlService = new SqlService();
