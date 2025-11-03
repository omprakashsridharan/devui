import { useState, useEffect, useCallback } from 'react';
import {
  Box,
  Typography,
  Tabs,
  Tab,
  Paper,
  TextField,
  Button,
  CircularProgress,
  Alert,
  Chip,
  List,
  ListItem,
  ListItemButton,
  ListItemText,
  ListItemIcon,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  IconButton,
  Tooltip,
  InputAdornment,
  Select,
  MenuItem,
  FormControl,
  Pagination,
} from '@mui/material';
import {
  Clear as ClearIcon,
  TableChart as TableIcon,
  Refresh as RefreshIcon,
  CheckCircle as ConnectedIcon,
  Error as ErrorIcon,
  ExpandMore as ExpandMoreIcon,
  ExpandLess as ExpandLessIcon,
  Visibility as ShowDataIcon,
  Search as SearchIcon,
  Link as LinkIcon,
  Save as SaveIcon,
  Edit as EditIcon,
  Check as CheckIcon,
  Close as CloseIcon,
} from '@mui/icons-material';
import { sqlService, type SqlConnection } from '../services/sqlService';

interface Table {
  name: string;
  schema: string;
  columns: Array<{
    name: string;
    data_type: string;
    is_nullable: boolean;
    is_primary_key: boolean;
    default_value: string | null;
    enum_values?: string[] | null;
    foreign_key?: {
      referenced_table: string;
      referenced_schema: string;
      referenced_column: string;
    } | null;
  }>;
}


const SqlEditor = () => {
  const [connections, setConnections] = useState<SqlConnection[]>([]);
  const [selectedConnection, setSelectedConnection] = useState<SqlConnection | null>(null);
  const [tables, setTables] = useState<Table[]>([]);
  const [loading, setLoading] = useState(true);
  const [connectionsLoading, setConnectionsLoading] = useState(true);
  const [expandedTables, setExpandedTables] = useState<Set<string>>(new Set());
  const [tableData, setTableData] = useState<{
    columns: string[];
    data: Record<string, unknown>[];
    total_rows: number;
    column_info: Array<{
      name: string;
      data_type: string;
      is_nullable: boolean;
      is_primary_key: boolean;
      default_value: string | null;
      enum_values?: string[] | null;
      foreign_key?: {
        referenced_table: string;
        referenced_schema: string;
        referenced_column: string;
      } | null;
    }>;
  } | null>(null);
  const [loadingTableData, setLoadingTableData] = useState(false);
  const [filters, setFilters] = useState<Record<string, string>>({});
  const [currentTableName, setCurrentTableName] = useState<string | null>(null);
  const [tableDataError, setTableDataError] = useState<string | null>(null);
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(10);
  const [foreignKeyFilter, setForeignKeyFilter] = useState<{
    column: string;
    value: string;
    referencedTable: string;
    referencedSchema: string;
  } | null>(null);

  // Edit state management
  const [editedData, setEditedData] = useState<Record<number, Record<string, unknown>>>({});
  const [originalData, setOriginalData] = useState<Record<number, Record<string, unknown>>>({});
  const [hasChanges, setHasChanges] = useState(false);
  const [activeEditingCell, setActiveEditingCell] = useState<{ rowIndex: number; column: string } | null>(null);
  const [editingCellValue, setEditingCellValue] = useState<unknown>(null);

  const loadConnections = async () => {
    try {
      setConnectionsLoading(true);
      const connectionsData = await sqlService.getConnections();
      setConnections(connectionsData);

      // Select first connection if available
      if (connectionsData.length > 0) {
        setSelectedConnection(connectionsData[0]);
      }
    } catch (error) {
      console.error('Failed to load connections:', error);
    } finally {
      setConnectionsLoading(false);
    }
  };

  const loadTables = useCallback(async () => {
    if (!selectedConnection) return;

    try {
      setLoading(true);
      const connectionId = getConnectionId(selectedConnection);
      const tablesData = await sqlService.getTables(connectionId);
      setTables(tablesData);
    } catch (error) {
      console.error('Failed to load tables:', error);
    } finally {
      setLoading(false);
    }
  }, [selectedConnection]);

  useEffect(() => {
    loadConnections();
  }, []);

  useEffect(() => {
    if (selectedConnection) {
      loadTables();
    }
  }, [selectedConnection, loadTables]);

  const handleConnectionChange = (connection: SqlConnection) => {
    setSelectedConnection(connection);
  };

  const getStatusIcon = (status?: string) => {
    switch (status) {
      case 'connected':
        return <ConnectedIcon color="success" />;
      case 'error':
        return <ErrorIcon color="error" />;
      default:
        // For database connections without explicit status, show a neutral icon
        return <TableIcon color="action" />;
    }
  };


  // Generate a unique ID for connections that don't have one
  const getConnectionId = (connection: SqlConnection) => {
    return connection.id || connection.name;
  };

  // Toggle table expansion
  const toggleTableExpansion = (tableName: string) => {
    setExpandedTables(prev => {
      const newSet = new Set(prev);
      if (newSet.has(tableName)) {
        newSet.delete(tableName);
      } else {
        newSet.add(tableName);
      }
      return newSet;
    });
  };

  // Load table data
  const loadTableData = async (
    tableName: string,
    appliedFilters?: Record<string, string>,
    pageNum?: number,
    pageSizeNum?: number
  ) => {
    if (!selectedConnection) return;

    try {
      setLoadingTableData(true);
      setTableDataError(null); // Clear any previous errors
      setTableData(null); // Clear previous data immediately
      setCurrentTableName(tableName);

      // Clear foreign key filter if switching to a different table
      // or if filters are explicitly cleared (empty object passed)
      const filtersToUse = appliedFilters ?? filters;
      if (tableName !== foreignKeyFilter?.referencedTable) {
        setForeignKeyFilter(null);
      } else if (appliedFilters !== undefined && Object.keys(appliedFilters).length === 0) {
        // Only clear if explicitly cleared (not just using empty filters state)
        setForeignKeyFilter(null);
      }

      const connectionId = getConnectionId(selectedConnection);
      const currentPage = pageNum ?? page;
      const currentPageSize = pageSizeNum ?? pageSize;
      const data = await sqlService.getTableData(
        connectionId,
        tableName,
        filtersToUse,
        currentPage,
        currentPageSize
      );
      setTableData(data);

      // Create snapshot of original data for change tracking
      const originalSnapshot: Record<number, Record<string, unknown>> = {};
      data.data.forEach((row, index) => {
        originalSnapshot[index] = { ...row };
      });
      setOriginalData(originalSnapshot);
      setEditedData({});
      setHasChanges(false);
      setActiveEditingCell(null);
      setEditingCellValue(null);

      // Update page state if explicitly provided
      if (pageNum !== undefined) {
        setPage(pageNum);
      }
      if (pageSizeNum !== undefined) {
        setPageSize(pageSizeNum);
      }
    } catch (error) {
      console.error('Failed to load table data:', error);
      setTableData(null); // Clear data on error
      setTableDataError(error instanceof Error ? error.message : 'Failed to load table data');
    } finally {
      setLoadingTableData(false);
    }
  };

  // Apply filters
  const applyFilters = () => {
    if (currentTableName) {
      setPage(1); // Reset to first page when applying filters
      loadTableData(currentTableName, filters, 1, pageSize);
    }
  };

  // Clear filters
  const clearFilters = () => {
    setFilters({});
    setForeignKeyFilter(null); // Clear foreign key filter indication
    setTableDataError(null); // Clear any errors when clearing filters
    setPage(1); // Reset to first page when clearing filters
    if (currentTableName) {
      loadTableData(currentTableName, {}, 1, pageSize);
    }
  };

  // Handle page change
  const handlePageChange = (_event: React.ChangeEvent<unknown> | React.MouseEvent<HTMLButtonElement> | null, value: number) => {
    setPage(value);
    if (currentTableName) {
      loadTableData(currentTableName, filters, value, pageSize);
    }
  };

  // Handle page size change
  const handlePageSizeChange = (event: { target: { value: number | string } }) => {
    const value = event.target.value;
    const newPageSize = typeof value === 'string' ? parseInt(value, 10) : value;
    if (typeof newPageSize === 'number') {
      setPageSize(newPageSize);
      setPage(1); // Reset to first page when changing page size
      if (currentTableName) {
        loadTableData(currentTableName, filters, 1, newPageSize);
      }
    }
  };

  // Navigate to referenced table for foreign key values
  const handleForeignKeyClick = (foreignKey: { referenced_table: string; referenced_schema: string; referenced_column: string }, value: string) => {
    if (!value || value === 'null' || value === '') {
      return;
    }

    // Load the referenced table
    const referencedTable = foreignKey.referenced_table;
    setPage(1);
    // Set filter to show the row with the matching foreign key value
    const filtersForReferencedTable: Record<string, string> = {
      [foreignKey.referenced_column]: value,
    };
    // Update filters state so filter inputs show the foreign key filter value
    setFilters(filtersForReferencedTable);
    // Track foreign key filter for indication
    setForeignKeyFilter({
      column: foreignKey.referenced_column,
      value: value,
      referencedTable: referencedTable,
      referencedSchema: foreignKey.referenced_schema,
    });
    loadTableData(referencedTable, filtersForReferencedTable, 1, pageSize);
  };

  // Clear foreign key filter
  const clearForeignKeyFilter = () => {
    setForeignKeyFilter(null);
    setFilters({});
    if (currentTableName) {
      loadTableData(currentTableName, {}, 1, pageSize);
    }
  };

  // Update filter for a specific column
  const updateFilter = (column: string, value: string) => {
    setFilters(prev => ({
      ...prev,
      [column]: value,
    }));
  };

  // Handle cell edit
  const handleCellEdit = (rowIndex: number, column: string, value: unknown, columnInfo?: { is_nullable: boolean; data_type: string }) => {
    // Convert value based on type and nullability
    let processedValue: unknown = value;

    if (value === '' && columnInfo?.is_nullable) {
      processedValue = null;
    } else if (columnInfo?.data_type) {
      const inputType = getInputType(columnInfo.data_type);
      if (inputType === 'number' && value !== null && value !== undefined && value !== '') {
        const numValue = Number(value);
        processedValue = isNaN(numValue) ? value : numValue;
      } else if (inputType === 'boolean') {
        if (value === 'true' || value === true) {
          processedValue = true;
        } else if (value === 'false' || value === false) {
          processedValue = false;
        } else if (value === '' && columnInfo.is_nullable) {
          processedValue = null;
        }
      }
    }

    // Update edited data
    setEditedData(prev => {
      const newEdited = { ...prev };
      if (!newEdited[rowIndex]) {
        newEdited[rowIndex] = { ...originalData[rowIndex] };
      }
      newEdited[rowIndex] = { ...newEdited[rowIndex], [column]: processedValue };
      return newEdited;
    });
  };

  // Detect changes by comparing editedData with originalData
  useEffect(() => {
    if (!tableData || Object.keys(editedData).length === 0) {
      setHasChanges(false);
      return;
    }

    let changesDetected = false;
    for (const [rowIndexStr, editedRow] of Object.entries(editedData)) {
      const rowIndex = parseInt(rowIndexStr, 10);
      const originalRow = originalData[rowIndex];

      if (!originalRow) continue;

      for (const column of tableData.columns) {
        const originalValue = originalRow[column];
        const editedValue = editedRow[column];

        // Compare values (handle null/undefined)
        if (originalValue !== editedValue) {
          // Normalize for comparison
          const orig = originalValue === null || originalValue === undefined ? null : String(originalValue);
          const edit = editedValue === null || editedValue === undefined ? null : String(editedValue);

          if (orig !== edit) {
            changesDetected = true;
            break;
          }
        }
      }

      if (changesDetected) break;
    }

    setHasChanges(changesDetected);
  }, [editedData, originalData, tableData]);

  // Prepare request body for committing changes
  const prepareCommitRequestBody = () => {
    if (!selectedConnection || !currentTableName || !tableData) {
      return null;
    }

    const connectionId = getConnectionId(selectedConnection);
    const changes: Array<{
      original_row: Record<string, unknown>;
      updated_row: Record<string, unknown>;
      primary_key_values: Record<string, unknown>;
    }> = [];

    // Iterate through edited rows
    for (const [rowIndexStr, editedRow] of Object.entries(editedData)) {
      const rowIndex = parseInt(rowIndexStr, 10);
      const originalRow = originalData[rowIndex];

      if (!originalRow) continue;

      // Check if row actually has changes
      let hasRowChanges = false;
      const updatedRow: Record<string, unknown> = { ...originalRow };

      for (const column of tableData.columns) {
        const originalValue = originalRow[column];
        const editedValue = editedRow[column];

        if (originalValue !== editedValue) {
          // Normalize for comparison
          const orig = originalValue === null || originalValue === undefined ? null : String(originalValue);
          const edit = editedValue === null || editedValue === undefined ? null : String(editedValue);

          if (orig !== edit) {
            hasRowChanges = true;
            updatedRow[column] = editedValue;
          }
        }
      }

      if (hasRowChanges) {
        // Extract primary key values from original row for WHERE clause
        const primaryKeyValues: Record<string, unknown> = {};
        tableData.column_info.forEach(colInfo => {
          if (colInfo.is_primary_key) {
            primaryKeyValues[colInfo.name] = originalRow[colInfo.name];
          }
        });

        changes.push({
          original_row: { ...originalRow },
          updated_row: updatedRow,
          primary_key_values: primaryKeyValues,
        });
      }
    }

    return {
      connectionId,
      table_name: currentTableName,
      changes,
    };
  };

  // Handle commit changes button click
  const handleCommitChanges = async () => {
    const requestBody = prepareCommitRequestBody();
    if (!requestBody || !selectedConnection || !currentTableName) {
      return;
    }

    try {
      const connectionId = getConnectionId(selectedConnection);
      await sqlService.updateTable(
        connectionId,
        requestBody.table_name,
        requestBody.changes
      );

      // Clear edited data and refresh table data
      setEditedData({});
      setOriginalData({});
      setHasChanges(false);

      // Reload table data to show updated values
      await loadTableData(currentTableName);

      // Show success message (you might want to add a toast notification here)
      console.log('Changes committed successfully');
    } catch (error) {
      console.error('Failed to commit changes:', error);
      // Show error message (you might want to add a toast notification here)
      alert(`Failed to commit changes: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  };

  // Helper function to determine input type based on data_type
  const getInputType = (dataType: string): 'enum' | 'date' | 'datetime' | 'time' | 'boolean' | 'number' | 'text' => {
    const normalizedType = dataType.toLowerCase();

    // Check for enum (USER-DEFINED with enum_values)
    if (normalizedType === 'user-defined') {
      return 'enum';
    }

    // Date/time types
    if (normalizedType.includes('date') && normalizedType.includes('time')) {
      return 'datetime';
    }
    if (normalizedType.includes('date') && !normalizedType.includes('time')) {
      return 'date';
    }
    if (normalizedType.includes('time')) {
      return 'time';
    }

    // Boolean types
    if (normalizedType === 'boolean' || normalizedType === 'bool') {
      return 'boolean';
    }

    // Numeric types
    if (normalizedType.includes('int') ||
        normalizedType.includes('numeric') ||
        normalizedType.includes('decimal') ||
        normalizedType.includes('float') ||
        normalizedType.includes('double') ||
        normalizedType.includes('real') ||
        normalizedType === 'smallint' ||
        normalizedType === 'bigint') {
      return 'number';
    }

    // Default to text
    return 'text';
  };

  // Render dynamic filter input based on column type
  const renderFilterInput = (column: string, columnInfo: { name: string; data_type: string; is_nullable: boolean; is_primary_key: boolean; default_value: string | null; enum_values?: string[] | null } | undefined) => {
    if (!columnInfo) {
      return (
        <TextField
          fullWidth
          size="small"
          placeholder={`Filter ${column}`}
          value={filters[column] || ''}
          onChange={(e) => updateFilter(column, e.target.value)}
          sx={textFieldStyles}
        />
      );
    }

    let inputType = getInputType(columnInfo.data_type);
    const currentValue = filters[column] || '';

    // Handle enum type first (before switch)
    if (inputType === 'enum') {
      if (columnInfo.enum_values && columnInfo.enum_values.length > 0) {
        return (
          <FormControl fullWidth size="small" sx={{ minHeight: 32 }}>
            <Select
              value={currentValue}
              onChange={(e) => updateFilter(column, e.target.value)}
              displayEmpty
              sx={{
                fontSize: '0.75rem',
                height: '32px',
                '& .MuiSelect-select': {
                  py: '6px',
                  px: '8px',
                },
              }}
            >
              <MenuItem value="">
                <em>All {column}</em>
              </MenuItem>
              {columnInfo.enum_values.map((value) => (
                <MenuItem key={value} value={value}>
                  {value}
                </MenuItem>
              ))}
            </Select>
          </FormControl>
        );
      }
      // If no enum values, treat as text
      inputType = 'text';
    }

    switch (inputType) {

      case 'date':
        return (
          <TextField
            fullWidth
            size="small"
            type="date"
            placeholder={`Filter ${column}`}
            value={currentValue}
            onChange={(e) => updateFilter(column, e.target.value)}
            InputLabelProps={{ shrink: true }}
            sx={textFieldStyles}
          />
        );

      case 'datetime':
        return (
          <TextField
            fullWidth
            size="small"
            type="datetime-local"
            placeholder={`Filter ${column}`}
            value={currentValue}
            onChange={(e) => updateFilter(column, e.target.value)}
            InputLabelProps={{ shrink: true }}
            sx={textFieldStyles}
          />
        );

      case 'time':
        return (
          <TextField
            fullWidth
            size="small"
            type="time"
            placeholder={`Filter ${column}`}
            value={currentValue}
            onChange={(e) => updateFilter(column, e.target.value)}
            InputLabelProps={{ shrink: true }}
            sx={textFieldStyles}
          />
        );

      case 'boolean':
        return (
          <FormControl fullWidth size="small" sx={{ minHeight: 32 }}>
            <Select
              value={currentValue}
              onChange={(e) => updateFilter(column, e.target.value)}
              displayEmpty
              sx={{
                fontSize: '0.75rem',
                height: '32px',
                '& .MuiSelect-select': {
                  py: '6px',
                  px: '8px',
                },
              }}
            >
              <MenuItem value="">
                <em>All {column}</em>
              </MenuItem>
              <MenuItem value="true">True</MenuItem>
              <MenuItem value="false">False</MenuItem>
            </Select>
          </FormControl>
        );

      case 'number':
        return (
          <TextField
            fullWidth
            size="small"
            type="number"
            placeholder={`Filter ${column}`}
            value={currentValue}
            onChange={(e) => updateFilter(column, e.target.value)}
            InputProps={{
              startAdornment: (
                <InputAdornment position="start">
                  <SearchIcon fontSize="small" color="inherit" />
                </InputAdornment>
              ),
            }}
            sx={textFieldStyles}
          />
        );

      default: // text
        return (
          <TextField
            fullWidth
            size="small"
            placeholder={`Filter ${column} (${columnInfo.data_type}${columnInfo.is_primary_key ? ', PK' : ''})`}
            value={currentValue}
            onChange={(e) => updateFilter(column, e.target.value)}
            InputProps={{
              startAdornment: (
                <InputAdornment position="start">
                  <SearchIcon fontSize="small" color="inherit" />
                </InputAdornment>
              ),
            }}
            sx={textFieldStyles}
          />
        );
    }
  };

  // Handle cell edit activation
  const handleCellClick = (rowIndex: number, column: string, value: unknown) => {
    setActiveEditingCell({ rowIndex, column });
    setEditingCellValue(value);
  };

  // Handle saving cell edit
  const handleCellSave = (rowIndex: number, column: string, columnInfo?: { is_nullable: boolean; data_type: string }) => {
    if (editingCellValue !== originalData[rowIndex]?.[column]) {
      handleCellEdit(rowIndex, column, editingCellValue, columnInfo);
    }
    setActiveEditingCell(null);
    setEditingCellValue(null);
  };

  // Handle canceling cell edit
  const handleCellCancel = () => {
    setActiveEditingCell(null);
    setEditingCellValue(null);
  };

  // Render editable cell input based on column type
  const renderEditableCell = (
    rowIndex: number,
    column: string,
    value: unknown,
    columnInfo?: { name: string; data_type: string; is_nullable: boolean; is_primary_key: boolean; enum_values?: string[] | null; foreign_key?: { referenced_table: string; referenced_schema: string; referenced_column: string } | null }
  ) => {
    // Check if cell is currently being edited
    const isEditing = activeEditingCell?.rowIndex === rowIndex && activeEditingCell?.column === column;

    // Check if cell has been edited (saved changes)
    const editedRow = editedData[rowIndex];
    const cellValue = editedRow && Object.prototype.hasOwnProperty.call(editedRow, column) ? editedRow[column] : value;
    const isEdited = editedRow && Object.prototype.hasOwnProperty.call(editedRow, column) && cellValue !== originalData[rowIndex]?.[column];

    // Primary key and foreign key columns are read-only
    if (columnInfo?.is_primary_key || columnInfo?.foreign_key) {
      const stringValue = cellValue === null || cellValue === undefined || cellValue === '' ? 'null' : String(cellValue);
      const displayValue = stringValue.length > 50 ? stringValue.substring(0, 50) + '...' : stringValue;

      if (columnInfo.foreign_key) {
        return (
          <Tooltip
            title={`Click to view ${columnInfo.foreign_key.referenced_schema}.${columnInfo.foreign_key.referenced_table}`}
            arrow
          >
            <Typography
              variant="body2"
              onClick={() => handleForeignKeyClick(columnInfo.foreign_key!, stringValue)}
              sx={{
                color: 'primary.main',
                cursor: 'pointer',
                textDecoration: 'underline',
                '&:hover': {
                  color: 'primary.dark',
                  textDecoration: 'underline',
                },
              }}
              title={stringValue}
            >
              {displayValue}
            </Typography>
          </Tooltip>
        );
      }

      return (
        <Typography variant="body2" title={stringValue} sx={{ fontFamily: 'monospace' }}>
          {displayValue}
        </Typography>
      );
    }

    // If not editing, show readonly view with edit icon on hover
    if (!isEditing) {
      const displayValue = cellValue === null || cellValue === undefined || cellValue === ''
        ? <em style={{ color: '#999' }}>null</em>
        : String(cellValue).length > 50
          ? String(cellValue).substring(0, 50) + '...'
          : String(cellValue);

      return (
        <Box
          sx={{
            position: 'relative',
            display: 'flex',
            alignItems: 'center',
            width: '100%',
            minHeight: '32px',
            px: 1,
            py: 0.5,
            cursor: 'pointer',
            borderRadius: 1,
            '&:hover': {
              backgroundColor: 'action.hover',
              '& .edit-icon': {
                opacity: 1,
              },
            },
            ...(isEdited && {
              backgroundColor: 'action.selected',
              borderLeft: '3px solid',
              borderColor: 'warning.main',
            }),
          }}
          onClick={() => handleCellClick(rowIndex, column, cellValue)}
        >
          <Typography
            variant="body2"
            sx={{
              flex: 1,
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
              fontFamily: cellValue === null || cellValue === undefined || cellValue === '' ? 'inherit' : 'monospace',
            }}
            title={cellValue === null || cellValue === undefined || cellValue === '' ? 'null' : String(cellValue)}
          >
            {displayValue}
          </Typography>
          <EditIcon
            className="edit-icon"
            sx={{
              opacity: 0,
              transition: 'opacity 0.2s',
              fontSize: '16px',
              color: 'text.secondary',
              ml: 1,
            }}
          />
        </Box>
      );
    }

    // If editing, show input with check/cancel buttons
    const currentEditingValue = editingCellValue === null || editingCellValue === undefined || editingCellValue === '' ? '' : String(editingCellValue);
    const hasChanged = String(editingCellValue || '') !== String(originalData[rowIndex]?.[column] || '');
    const inputType = columnInfo ? getInputType(columnInfo.data_type) : 'text';

    // Render input based on type
    const renderInput = () => {
      // Handle enum type
      if (inputType === 'enum' && columnInfo?.enum_values && columnInfo.enum_values.length > 0) {
        return (
          <Select
            value={currentEditingValue}
            onChange={(e) => setEditingCellValue(e.target.value)}
            displayEmpty
            autoFocus
            size="small"
            sx={{
              fontSize: '0.75rem',
              height: '32px',
              width: '100%',
              '& .MuiSelect-select': {
                py: '6px',
                px: '8px',
              },
            }}
          >
            {columnInfo.is_nullable && (
              <MenuItem value="">
                <em>null</em>
              </MenuItem>
            )}
            {columnInfo.enum_values.map((enumValue) => (
              <MenuItem key={enumValue} value={enumValue}>
                {enumValue}
              </MenuItem>
            ))}
          </Select>
        );
      }

      switch (inputType) {
        case 'boolean':
          return (
            <Select
              value={currentEditingValue}
              onChange={(e) => setEditingCellValue(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Escape') {
                  handleCellCancel();
                }
              }}
              displayEmpty
              autoFocus
              size="small"
              sx={{
                fontSize: '0.75rem',
                height: '32px',
                width: '100%',
                '& .MuiSelect-select': {
                  py: '6px',
                  px: '8px',
                },
              }}
            >
              {columnInfo?.is_nullable && (
                <MenuItem value="">
                  <em>null</em>
                </MenuItem>
              )}
              <MenuItem value="true">True</MenuItem>
              <MenuItem value="false">False</MenuItem>
            </Select>
          );

        case 'date':
          return (
            <TextField
              type="date"
              value={currentEditingValue}
              onChange={(e) => setEditingCellValue(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && hasChanged) {
                  handleCellSave(rowIndex, column, columnInfo);
                } else if (e.key === 'Escape') {
                  handleCellCancel();
                }
              }}
              InputLabelProps={{ shrink: true }}
              autoFocus
              size="small"
              fullWidth
              sx={textFieldStyles}
            />
          );

        case 'datetime':
          return (
            <TextField
              type="datetime-local"
              value={currentEditingValue}
              onChange={(e) => setEditingCellValue(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && hasChanged) {
                  handleCellSave(rowIndex, column, columnInfo);
                } else if (e.key === 'Escape') {
                  handleCellCancel();
                }
              }}
              InputLabelProps={{ shrink: true }}
              autoFocus
              size="small"
              fullWidth
              sx={textFieldStyles}
            />
          );

        case 'time':
          return (
            <TextField
              type="time"
              value={currentEditingValue}
              onChange={(e) => setEditingCellValue(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && hasChanged) {
                  handleCellSave(rowIndex, column, columnInfo);
                } else if (e.key === 'Escape') {
                  handleCellCancel();
                }
              }}
              InputLabelProps={{ shrink: true }}
              autoFocus
              size="small"
              fullWidth
              sx={textFieldStyles}
            />
          );

        case 'number':
          return (
            <TextField
              type="number"
              value={currentEditingValue}
              onChange={(e) => setEditingCellValue(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && hasChanged) {
                  handleCellSave(rowIndex, column, columnInfo);
                } else if (e.key === 'Escape') {
                  handleCellCancel();
                }
              }}
              autoFocus
              size="small"
              fullWidth
              sx={textFieldStyles}
            />
          );

        default: // text
          return (
            <TextField
              value={currentEditingValue}
              onChange={(e) => setEditingCellValue(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && hasChanged) {
                  handleCellSave(rowIndex, column, columnInfo);
                } else if (e.key === 'Escape') {
                  handleCellCancel();
                }
              }}
              placeholder={columnInfo?.is_nullable ? 'null' : ''}
              autoFocus
              size="small"
              fullWidth
              sx={textFieldStyles}
            />
          );
      }
    };

    return (
      <Box
        sx={{
          display: 'flex',
          alignItems: 'center',
          gap: 0.5,
          width: '100%',
        }}
      >
        <Box sx={{ flex: 1 }}>
          {renderInput()}
        </Box>
        {hasChanged && (
          <IconButton
            size="small"
            onClick={() => handleCellSave(rowIndex, column, columnInfo)}
            sx={{
              color: 'success.main',
              '&:hover': {
                backgroundColor: 'success.light',
                color: 'success.dark',
              },
            }}
          >
            <CheckIcon fontSize="small" />
          </IconButton>
        )}
        <IconButton
          size="small"
          onClick={handleCellCancel}
          sx={{
            color: 'text.secondary',
            '&:hover': {
              backgroundColor: 'error.light',
              color: 'error.main',
            },
          }}
        >
          <CloseIcon fontSize="small" />
        </IconButton>
      </Box>
    );
  };

  // Shared styles for text fields
  const textFieldStyles = {
    '& .MuiOutlinedInput-root': {
      fontSize: '0.75rem',
      height: '32px',
      color: 'text.primary',
      backgroundColor: 'background.default',
      '& .MuiOutlinedInput-notchedOutline': {
        borderColor: 'divider',
      },
      '&:hover .MuiOutlinedInput-notchedOutline': {
        borderColor: 'text.secondary',
      },
      '&.Mui-focused .MuiOutlinedInput-notchedOutline': {
        borderColor: 'primary.main',
      },
    },
    '& .MuiInputBase-input': {
      padding: '6px 8px',
      '::placeholder': {
        color: 'text.secondary',
        opacity: 1,
      },
    },
    '& .MuiInputAdornment-root svg': {
      color: 'text.secondary',
    },
  };

  if (connectionsLoading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
        <Typography variant="h6" sx={{ ml: 2 }}>
          Loading SQL connections...
        </Typography>
      </Box>
    );
  }

  if (connections.length === 0) {
    return (
      <Box p={3}>
        <Alert severity="warning">
          No SQL connections found. Please configure connections first.
        </Alert>
      </Box>
    );
  }

  return (
    <Box sx={{ height: '100%', minHeight: 0, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
      {/* Header */}
      <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider' }}>
        <Typography variant="h5" gutterBottom>
          SQL Editor
        </Typography>

        {/* Connection Tabs */}
        <Tabs
          value={selectedConnection ? getConnectionId(selectedConnection) : false}
          onChange={(_, value) => {
            const connection = connections.find(c => getConnectionId(c) === value);
            if (connection) handleConnectionChange(connection);
          }}
          variant="scrollable"
          scrollButtons="auto"
        >
          {connections.map((connection) => (
            <Tab
              key={getConnectionId(connection)}
              value={getConnectionId(connection)}
              label={
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                  {getStatusIcon(connection.status)}
                  <Typography variant="body2">{connection.name}</Typography>
                  <Chip
                    label={connection.database_type || 'unknown'}
                    color="primary"
                    size="small"
                  />
                </Box>
              }
            />
          ))}
        </Tabs>
      </Box>

      <Box sx={{ flex: '1 1 auto', minHeight: 0, display: 'flex', overflow: 'hidden' }}>
        {/* Sidebar */}
        <Paper sx={{ width: 300, m: 2, display: 'flex', flexDirection: 'column', flexShrink: 0 }}>
          <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider' }}>
            <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <Typography variant="h6">
            Database Tables ({tables.length})
              </Typography>
              <Tooltip title="Refresh">
                <IconButton size="small" onClick={loadTables}>
                  <RefreshIcon />
                </IconButton>
              </Tooltip>
            </Box>
            {selectedConnection && (
              <Typography variant="body2" color="text.secondary">
                {selectedConnection.name} ({selectedConnection.database_type})
              </Typography>
            )}
          </Box>

          <Box sx={{ flexGrow: 1, overflow: 'auto' }}>
            {loading ? (
              <Box sx={{ p: 2, textAlign: 'center' }}>
                <CircularProgress size={24} />
                <Typography variant="body2" sx={{ mt: 1 }}>
                  Loading tables...
                </Typography>
              </Box>
            ) : tables.length === 0 ? (
              <Box sx={{ p: 2 }}>
                <Typography variant="body2" color="text.secondary">
                  No tables found or database not connected
                </Typography>
              </Box>
            ) : (
              <List dense>
                {tables.map((table, index) => {
                  const isExpanded = expandedTables.has(table.name);
                  return (
                    <ListItem key={index} sx={{ flexDirection: 'column', alignItems: 'stretch', p: 0 }}>
                      <ListItemButton
                        onClick={() => toggleTableExpansion(table.name)}
                        sx={{
                          display: 'flex',
                          alignItems: 'center',
                          width: '100%',
                          py: 1,
                          px: 2
                        }}
                      >
                        <ListItemIcon sx={{ minWidth: 40 }}>
                          <TableIcon />
                        </ListItemIcon>
                        <ListItemText
                          primary={table.name}
                          secondary={`Schema: ${table.schema} • ${table.columns.length} columns`}
                        />
                        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                          <Tooltip title="Show Table Data">
                            <IconButton
                              size="small"
                              onClick={(e) => {
                                e.stopPropagation();
                                setPage(1); // Reset to first page when loading new table
                                loadTableData(table.name, {}, 1, pageSize);
                              }}
                              sx={{
                                color: 'primary.main',
                                '&:hover': {
                                  backgroundColor: 'primary.light',
                                  color: 'primary.contrastText'
                                }
                              }}
                            >
                              <ShowDataIcon />
                            </IconButton>
                          </Tooltip>
                          <IconButton size="small">
                            {isExpanded ? <ExpandLessIcon /> : <ExpandMoreIcon />}
                          </IconButton>
                        </Box>
                      </ListItemButton>

                      {isExpanded && (
                        <Box sx={{ ml: 4, maxHeight: 200, overflow: 'auto', borderLeft: 1, borderColor: 'divider', pl: 2 }}>
                          <List dense>
                            {table.columns.map((column, colIndex) => (
                              <ListItem key={colIndex} sx={{ py: 0.5 }}>
                                <ListItemText
                                  primary={
                                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                                      <Typography variant="body2" fontWeight="medium">
                                        {column.name}
                                      </Typography>
                                      {column.is_primary_key && (
                                        <Chip label="PK" size="small" color="primary" />
                                      )}
                                      {column.foreign_key && (
                                        <Chip label="FK" size="small" color="info" />
                                      )}
                                      {!column.is_nullable && (
                                        <Chip label="NOT NULL" size="small" color="warning" />
                                      )}
                                    </Box>
                                  }
                                  secondary={
                                    <Typography variant="caption" color="text.secondary">
                                      {column.data_type}
                                      {column.foreign_key && ` • FK → ${column.foreign_key.referenced_schema}.${column.foreign_key.referenced_table}`}
                                      {column.default_value && ` • Default: ${column.default_value}`}
                                    </Typography>
                                  }
                                />
                              </ListItem>
                            ))}
                          </List>
                        </Box>
                      )}
                    </ListItem>
                  );
                })}
              </List>
            )}
          </Box>
        </Paper>

        {/* Main Content */}
        <Box sx={{ flexGrow: 1, minHeight: 0, display: 'flex', flexDirection: 'column', m: 2, overflow: 'hidden' }}>
          {/* Table Data Display */}
          {(tableData || tableDataError || loadingTableData) && (
            <Paper sx={{ mb: 2, flex: '1 1 auto', minHeight: 0, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
              <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider', flexShrink: 0 }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                  <Box>
                    <Typography variant="h6">Table Data</Typography>
                    {tableData && !tableDataError && (
                      <Typography variant="body2" color="text.secondary">
                        Showing {((page - 1) * pageSize) + 1} - {Math.min(page * pageSize, tableData.total_rows)} of {tableData.total_rows.toLocaleString()} rows • {tableData.columns.length} columns • Page {page} of {Math.ceil(tableData.total_rows / pageSize) || 1}
                      </Typography>
                    )}
                    {tableDataError && (
                      <Typography variant="body2" color="error">
                        Error loading table data
                      </Typography>
                    )}
                  </Box>
                  <Box sx={{ display: 'flex', gap: 1 }}>
                    {hasChanges && (
                      <Button
                        variant="contained"
                        color="success"
                        startIcon={<SaveIcon />}
                        onClick={handleCommitChanges}
                        disabled={loadingTableData || !selectedConnection || !currentTableName}
                        size="small"
                      >
                        Commit Changes
                      </Button>
                    )}
                    <Button
                      variant="contained"
                      startIcon={<SearchIcon />}
                      onClick={applyFilters}
                      disabled={loadingTableData || !currentTableName}
                      size="small"
                    >
                      Apply Filters
                    </Button>
                    <Button
                      variant="outlined"
                      startIcon={<ClearIcon />}
                      onClick={clearFilters}
                      disabled={loadingTableData || Object.keys(filters).length === 0}
                      size="small"
                    >
                      Clear All
                    </Button>
                  </Box>
                </Box>
              </Box>
              {/* Foreign Key Filter Indicator */}
              {foreignKeyFilter && currentTableName === foreignKeyFilter.referencedTable && (
                <Alert
                  severity="info"
                  icon={<LinkIcon />}
                  action={
                    <IconButton
                      aria-label="clear foreign key filter"
                      color="inherit"
                      size="small"
                      onClick={clearForeignKeyFilter}
                    >
                      <ClearIcon fontSize="inherit" />
                    </IconButton>
                  }
                  sx={{ m: 2, flexShrink: 0 }}
                >
                  <Box>
                    <Typography variant="body2" fontWeight="medium" gutterBottom>
                      Filtered by Foreign Key Navigation
                    </Typography>
                    <Typography variant="caption" color="text.secondary">
                      Showing rows where <strong>{foreignKeyFilter.column}</strong> = <strong>{foreignKeyFilter.value}</strong>
                      {' • '}Navigated from <strong>{foreignKeyFilter.referencedSchema}.{foreignKeyFilter.referencedTable}</strong>
                    </Typography>
                  </Box>
                </Alert>
              )}

              <Box sx={{ flex: '1 1 auto', minHeight: 0, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                {loadingTableData ? (
                  <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: 200 }}>
                    <CircularProgress />
                    <Typography variant="body2" sx={{ ml: 2 }}>
                      Loading table data...
                    </Typography>
                  </Box>
                ) : tableDataError ? (
                  <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: 200, p: 2 }}>
                    <Alert severity="error" sx={{ width: '100%' }}>
                      <Typography variant="body1" sx={{ fontWeight: 'bold', mb: 1 }}>
                        Failed to load table data
                      </Typography>
                      <Typography variant="body2">
                        {tableDataError}
                      </Typography>
                    </Alert>
                  </Box>
                ) : tableData ? (
                  <Box sx={{
                    flex: '1 1 auto',
                    minHeight: 0,
                    display: 'flex',
                    flexDirection: 'column',
                    border: '1px solid',
                    borderColor: 'divider',
                    borderRadius: 1,
                    backgroundColor: 'background.paper',
                    overflow: 'hidden'
                  }}>
                    <TableContainer
                      component="div"
                      sx={{
                        flex: '1 1 0%',
                        minHeight: 0,
                        overflow: 'auto',
                        position: 'relative'
                      }}
                    >
                      <Table size="small" stickyHeader sx={{ width: '100%' }}>
                        <TableHead>
                          <TableRow>
                            {tableData.columns.map((column, index) => {
                              const columnInfo = tableData.column_info.find(col => col.name === column);
                              return (
                                <TableCell
                                  key={index}
                                  sx={{
                                    fontWeight: 'bold',
                                    backgroundColor: 'primary.main',
                                    color: 'primary.contrastText',
                                    borderBottom: '2px solid',
                                    borderColor: 'primary.dark',
                                    minWidth: 200,
                                    whiteSpace: 'normal',
                                    verticalAlign: 'top',
                                    py: 1.5,
                                  }}
                                >
                                  <Tooltip
                                    title={
                                      <Box>
                                        <Typography variant="body2" sx={{ fontWeight: 'bold', mb: 0.5 }}>
                                          {column}
                                        </Typography>
                                        {columnInfo && (
                                          <>
                                            <Typography variant="caption" display="block">
                                              Type: {columnInfo.data_type}
                                            </Typography>
                                            <Typography variant="caption" display="block">
                                              Nullable: {columnInfo.is_nullable ? 'Yes' : 'No'}
                                            </Typography>
                                            {columnInfo.is_primary_key && (
                                              <Typography variant="caption" display="block">
                                                Primary Key
                                              </Typography>
                                            )}
                                            {columnInfo.foreign_key && (
                                              <Typography variant="caption" display="block">
                                                Foreign Key → {columnInfo.foreign_key.referenced_schema}.{columnInfo.foreign_key.referenced_table}({columnInfo.foreign_key.referenced_column})
                                              </Typography>
                                            )}
                                            {columnInfo.default_value && (
                                              <Typography variant="caption" display="block">
                                                Default: {columnInfo.default_value}
                                              </Typography>
                                            )}
                                            {columnInfo.enum_values && columnInfo.enum_values.length > 0 && (
                                              <Typography variant="caption" display="block" sx={{ mt: 0.5 }}>
                                                Enum Values: {columnInfo.enum_values.join(', ')}
                                              </Typography>
                                            )}
                                          </>
                                        )}
                                      </Box>
                                    }
                                    arrow
                                    placement="top"
                                  >
                                    <Box sx={{ display: 'flex', flexDirection: 'column', gap: 0.5 }}>
                                      <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5, flexWrap: 'wrap' }}>
                                        <Typography variant="body2" sx={{ fontWeight: 'bold' }}>
                                          {column}
                                        </Typography>
                                        {columnInfo?.is_primary_key && (
                                          <Chip
                                            label="PK"
                                            size="small"
                                            sx={{
                                              height: 18,
                                              fontSize: '0.65rem',
                                              fontWeight: 'bold',
                                              backgroundColor: 'warning.main',
                                              color: 'warning.contrastText',
                                            }}
                                          />
                                        )}
                                        {columnInfo?.foreign_key && (
                                          <Chip
                                            label="FK"
                                            size="small"
                                            sx={{
                                              height: 18,
                                              fontSize: '0.65rem',
                                              fontWeight: 'bold',
                                              backgroundColor: 'info.main',
                                              color: 'info.contrastText',
                                            }}
                                          />
                                        )}
                                        {columnInfo && !columnInfo.is_nullable && (
                                          <Chip
                                            label="NOT NULL"
                                            size="small"
                                            sx={{
                                              height: 18,
                                              fontSize: '0.65rem',
                                              backgroundColor: 'error.main',
                                              color: 'error.contrastText',
                                            }}
                                          />
                                        )}
                                      </Box>
                                      {columnInfo && (
                                        <Typography
                                          variant="caption"
                                          sx={{
                                            fontSize: '0.7rem',
                                            opacity: 0.9,
                                            fontFamily: 'monospace',
                                            display: 'block',
                                          }}
                                        >
                                          {columnInfo.data_type}
                                        </Typography>
                                      )}
                                    </Box>
                                  </Tooltip>
                                </TableCell>
                              );
                            })}
                          </TableRow>
                          <TableRow>
                            {tableData.columns.map((column, index) => {
                              const columnInfo = tableData.column_info.find(col => col.name === column);
                              const inputType = columnInfo ? getInputType(columnInfo.data_type) : 'text';

                              return (
                                <TableCell
                                  key={`filter-${index}`}
                                  sx={{
                                    padding: 1,
                                    backgroundColor: 'background.paper',
                                    borderBottom: '1px solid',
                                    borderColor: 'divider',
                                    minWidth: 200,
                                    whiteSpace: 'nowrap',
                                  }}
                                >
                                  <Tooltip
                                    title={
                                      columnInfo ? (
                                        <Box>
                                          <Typography variant="caption" display="block">
                                            Type: {columnInfo.data_type}
                                          </Typography>
                                          {!columnInfo.is_nullable && (
                                            <Typography variant="caption" display="block">
                                              Required field (NOT NULL)
                                            </Typography>
                                          )}
                                          {columnInfo.enum_values && columnInfo.enum_values.length > 0 && (
                                            <Typography variant="caption" display="block">
                                              Allowed values: {columnInfo.enum_values.join(', ')}
                                            </Typography>
                                          )}
                                          {(inputType === 'text' || inputType === 'number') && (
                                            <Typography variant="caption" display="block" sx={{ mt: 0.5, fontStyle: 'italic' }}>
                                              Use operators: eq:, gt:, lt:, contains:, etc.
                                            </Typography>
                                          )}
                                        </Box>
                                      ) : ''
                                    }
                                    arrow
                                    placement="top"
                                  >
                                    <Box>
                                      {renderFilterInput(column, columnInfo)}
                                    </Box>
                                  </Tooltip>
                                </TableCell>
                              );
                            })}
                          </TableRow>
                        </TableHead>
                        <TableBody>
                          {tableData.data.map((row, index) => {
                            // Check if this row matches the foreign key filter
                            const matchesForeignKeyFilter = foreignKeyFilter &&
                              currentTableName === foreignKeyFilter.referencedTable &&
                              row[foreignKeyFilter.column]?.toString() === foreignKeyFilter.value;

                            return (
                            <TableRow
                              key={index}
                              sx={{
                                '&:nth-of-type(odd)': {
                                  backgroundColor: matchesForeignKeyFilter
                                    ? 'info.light'
                                    : 'action.hover',
                                },
                                '&:hover': {
                                  backgroundColor: matchesForeignKeyFilter
                                    ? 'info.main'
                                    : 'action.selected',
                                },
                                ...(matchesForeignKeyFilter && {
                                  borderLeft: '4px solid',
                                  borderLeftColor: 'info.main',
                                  backgroundColor: 'info.light',
                                }),
                              }}
                            >
                              {tableData.columns.map((column, colIndex) => {
                                const columnInfo = tableData.column_info.find(col => col.name === column);
                                return (
                                  <TableCell
                                    key={colIndex}
                                    sx={{
                                      borderBottom: '1px solid',
                                      borderColor: 'divider',
                                      minWidth: 200,
                                      padding: '8px',
                                    }}
                                  >
                                    {renderEditableCell(index, column, row[column], columnInfo)}
                                  </TableCell>
                                );
                              })}
                            </TableRow>
                            );
                          })}
                        </TableBody>
                      </Table>
                    </TableContainer>
                  </Box>
                ) : null}
                {/* Pagination Controls */}
                {tableData && !tableDataError && tableData.total_rows > 0 && (
                  <Box
                    sx={{
                      p: 2,
                      borderTop: 1,
                      borderColor: 'divider',
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                      flexWrap: 'wrap',
                      gap: 2,
                    }}
                  >
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                      <Typography variant="body2" color="text.secondary">
                        Rows per page:
                      </Typography>
                      <Select
                        value={pageSize}
                        onChange={handlePageSizeChange}
                        size="small"
                        sx={{
                          minWidth: 80,
                          height: 32,
                          '& .MuiSelect-select': {
                            py: '4px',
                          },
                        }}
                      >
                        <MenuItem value={10}>10</MenuItem>
                        <MenuItem value={25}>25</MenuItem>
                        <MenuItem value={50}>50</MenuItem>
                        <MenuItem value={100}>100</MenuItem>
                      </Select>
                    </Box>
                    <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
                      <Typography variant="body2" color="text.secondary">
                        {((page - 1) * pageSize) + 1} - {Math.min(page * pageSize, tableData.total_rows)} of {tableData.total_rows.toLocaleString()}
                      </Typography>
                      <Pagination
                        count={Math.ceil(tableData.total_rows / pageSize) || 1}
                        page={page}
                        onChange={handlePageChange}
                        color="primary"
                        size="small"
                        showFirstButton
                        showLastButton
                        disabled={loadingTableData}
                      />
                    </Box>
                  </Box>
                )}
              </Box>
            </Paper>
          )}

        </Box>
      </Box>
    </Box>
  );
};

export default SqlEditor;
