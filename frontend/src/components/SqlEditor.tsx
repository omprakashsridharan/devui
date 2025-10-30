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
  } | null>(null);
  const [loadingTableData, setLoadingTableData] = useState(false);
  const [filters, setFilters] = useState<Record<string, string>>({});
  const [currentTableName, setCurrentTableName] = useState<string | null>(null);
  const [tableDataError, setTableDataError] = useState<string | null>(null);

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
  const loadTableData = async (tableName: string, appliedFilters?: Record<string, string>) => {
    if (!selectedConnection) return;

    try {
      setLoadingTableData(true);
      setTableDataError(null); // Clear any previous errors
      setTableData(null); // Clear previous data immediately
      setCurrentTableName(tableName);

      const connectionId = getConnectionId(selectedConnection);
      const data = await sqlService.getTableData(connectionId, tableName, appliedFilters || filters);
      setTableData(data);
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
      loadTableData(currentTableName, filters);
    }
  };

  // Clear filters
  const clearFilters = () => {
    setFilters({});
    setTableDataError(null); // Clear any errors when clearing filters
    if (currentTableName) {
      loadTableData(currentTableName, {});
    }
  };

  // Update filter for a specific column
  const updateFilter = (column: string, value: string) => {
    setFilters(prev => ({
      ...prev,
      [column]: value,
    }));
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
    <Box sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
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

      <Box sx={{ flexGrow: 1, display: 'flex', overflow: 'hidden' }}>
        {/* Sidebar */}
        <Paper sx={{ width: 300, m: 2, display: 'flex', flexDirection: 'column' }}>
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
                                loadTableData(table.name);
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
                                      {!column.is_nullable && (
                                        <Chip label="NOT NULL" size="small" color="warning" />
                                      )}
                                    </Box>
                                  }
                                  secondary={
                                    <Typography variant="caption" color="text.secondary">
                                      {column.data_type}
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
        <Box sx={{ flexGrow: 1, display: 'flex', flexDirection: 'column', m: 2, overflow: 'hidden' }}>
          {/* Table Data Display */}
          {(tableData || tableDataError || loadingTableData) && (
            <Paper sx={{ mb: 2, flexGrow: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
              <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider', flexShrink: 0 }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                  <Box>
                    <Typography variant="h6">Table Data</Typography>
                    {tableData && !tableDataError && (
                      <Typography variant="body2" color="text.secondary">
                        {tableData.data.length} rows • {tableData.columns.length} columns
                      </Typography>
                    )}
                    {tableDataError && (
                      <Typography variant="body2" color="error">
                        Error loading table data
                      </Typography>
                    )}
                  </Box>
                  <Box sx={{ display: 'flex', gap: 1 }}>
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

              <Box sx={{ flexGrow: 1, overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
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
                    flexGrow: 1,
                    overflow: 'auto',
                    border: '1px solid',
                    borderColor: 'divider',
                    borderRadius: 1,
                    backgroundColor: 'background.paper'
                  }}>
                    <TableContainer sx={{
                      minWidth: `${Math.max(tableData.columns.length * 200, 800)}px`,
                      width: 'max-content'
                    }}>
                      <Table size="small" sx={{ minWidth: '100%' }}>
                        <TableHead>
                          <TableRow>
                            {tableData.columns.map((column, index) => (
                              <TableCell
                                key={index}
                                sx={{
                                  fontWeight: 'bold',
                                  backgroundColor: 'primary.main',
                                  color: 'primary.contrastText',
                                  borderBottom: '2px solid',
                                  borderColor: 'primary.dark',
                                  minWidth: 200,
                                  width: 200,
                                  whiteSpace: 'nowrap',
                                }}
                              >
                                {column}
                              </TableCell>
                            ))}
                          </TableRow>
                          <TableRow>
                            {tableData.columns.map((column, index) => (
                              <TableCell
                                key={`filter-${index}`}
                                sx={{
                                  padding: 1,
                                  backgroundColor: 'background.paper',
                                  borderBottom: '1px solid',
                                  borderColor: 'divider',
                                  minWidth: 200,
                                  width: 200,
                                  whiteSpace: 'nowrap',
                                }}
                              >
                                <TextField
                                  fullWidth
                                  size="small"
                                  placeholder={`Filter ${column}`}
                                  value={filters[column] || ''}
                                  onChange={(e) => updateFilter(column, e.target.value)}
                                  InputProps={{
                                    startAdornment: (
                                      <InputAdornment position="start">
                                        <SearchIcon fontSize="small" color="inherit" />
                                      </InputAdornment>
                                    ),
                                  }}
                                  sx={{
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
                                  }}
                                />
                              </TableCell>
                            ))}
                          </TableRow>
                        </TableHead>
                        <TableBody>
                          {tableData.data.map((row, index) => (
                            <TableRow
                              key={index}
                              sx={{
                                '&:nth-of-type(odd)': {
                                  backgroundColor: 'action.hover',
                                },
                                '&:hover': {
                                  backgroundColor: 'action.selected',
                                },
                              }}
                            >
                              {tableData.columns.map((column, colIndex) => (
                                <TableCell
                                  key={colIndex}
                                  sx={{
                                    borderBottom: '1px solid',
                                    borderColor: 'divider',
                                    minWidth: 200,
                                    width: 200,
                                    whiteSpace: 'nowrap',
                                    overflow: 'hidden',
                                    textOverflow: 'ellipsis',
                                  }}
                                >
                                  {(() => {
                                    const value = row[column];
                                    if (value === null || value === undefined || value === '') {
                                      return (
                                        <Typography
                                          variant="body2"
                                          color="text.secondary"
                                          sx={{ fontStyle: 'italic' }}
                                        >
                                          null
                                        </Typography>
                                      );
                                    }

                                    const stringValue = String(value);
                                    if (stringValue.length > 50) {
                                      return (
                                        <Typography variant="body2" title={stringValue}>
                                          {stringValue.substring(0, 50)}...
                                        </Typography>
                                      );
                                    }

                                    return (
                                      <Typography variant="body2" title={stringValue}>
                                        {stringValue}
                                      </Typography>
                                    );
                                  })()}
                                </TableCell>
                              ))}
                            </TableRow>
                          ))}
                        </TableBody>
                      </Table>
                    </TableContainer>
                  </Box>
                ) : null}
              </Box>
            </Paper>
          )}

        </Box>
      </Box>
    </Box>
  );
};

export default SqlEditor;
