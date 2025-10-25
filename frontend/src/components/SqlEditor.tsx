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
} from '@mui/material';
import {
  PlayArrow as PlayIcon,
  Clear as ClearIcon,
  TableChart as TableIcon,
  Refresh as RefreshIcon,
  CheckCircle as ConnectedIcon,
  Error as ErrorIcon,
  Warning as WarningIcon,
  ExpandMore as ExpandMoreIcon,
  ExpandLess as ExpandLessIcon,
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

interface QueryResult {
  success: boolean;
  data?: Record<string, unknown>[];
  columns?: string[];
  message?: string;
  executionTime?: number;
}

const SqlEditor = () => {
  const [connections, setConnections] = useState<SqlConnection[]>([]);
  const [selectedConnection, setSelectedConnection] = useState<SqlConnection | null>(null);
  const [tables, setTables] = useState<Table[]>([]);
  const [loading, setLoading] = useState(true);
  const [connectionsLoading, setConnectionsLoading] = useState(true);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<QueryResult | null>(null);
  const [executing, setExecuting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [expandedTables, setExpandedTables] = useState<Set<string>>(new Set());

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
      setError('Failed to load SQL connections');
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
      setError('Failed to load database tables');
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

  const executeQuery = async () => {
    if (!query.trim() || !selectedConnection) return;

    setExecuting(true);
    setError(null);
    try {
      const connectionId = getConnectionId(selectedConnection);
      const result = await sqlService.executeQuery(connectionId, query);
      setResults(result);
    } catch (error) {
      console.error('Failed to execute query:', error);
      setError(error instanceof Error ? error.message : 'Failed to execute query');
      setResults({
        success: false,
        message: error instanceof Error ? error.message : 'Failed to execute query',
      });
    } finally {
      setExecuting(false);
    }
  };

  const clearQuery = () => {
    setQuery('');
    setResults(null);
    setError(null);
  };

  const handleConnectionChange = (connection: SqlConnection) => {
    setSelectedConnection(connection);
    setQuery('');
    setResults(null);
    setError(null);
  };

  const getStatusIcon = (status?: string) => {
    switch (status) {
      case 'connected':
        return <ConnectedIcon color="success" />;
      case 'error':
        return <ErrorIcon color="error" />;
      default:
        return <WarningIcon color="warning" />;
    }
  };

  const getStatusColor = (status?: string) => {
    switch (status) {
      case 'connected':
        return 'success';
      case 'error':
        return 'error';
      default:
        return 'warning';
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
                    label={connection.status || 'unknown'}
                    color={getStatusColor(connection.status) as 'success' | 'error' | 'warning'}
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
                        <IconButton size="small">
                          {isExpanded ? <ExpandLessIcon /> : <ExpandMoreIcon />}
                        </IconButton>
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
        <Box sx={{ flexGrow: 1, display: 'flex', flexDirection: 'column', m: 2 }}>
          {/* Query Editor */}
          <Paper sx={{ mb: 2, flexGrow: 1, display: 'flex', flexDirection: 'column' }}>
            <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider' }}>
              <Box sx={{ display: 'flex', gap: 1 }}>
                <Button
                  variant="contained"
                  startIcon={executing ? <CircularProgress size={16} /> : <PlayIcon />}
                  onClick={executeQuery}
                  disabled={executing || !query.trim() || !selectedConnection}
                >
                  {executing ? 'Executing...' : 'Execute Query'}
                </Button>
                <Button
                  variant="outlined"
                  startIcon={<ClearIcon />}
                  onClick={clearQuery}
                >
                  Clear
                </Button>
              </Box>
            </Box>

            <Box sx={{ flexGrow: 1, p: 2 }}>
              <TextField
                fullWidth
                multiline
                rows={8}
                variant="outlined"
                placeholder="Enter your SQL query here...\n\nExample:\nSELECT * FROM users LIMIT 10;"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                sx={{
                  '& .MuiInputBase-root': {
                    fontFamily: 'monospace',
                  },
                }}
              />
            </Box>
          </Paper>

          {/* Results */}
          <Paper sx={{ flexGrow: 1, display: 'flex', flexDirection: 'column' }}>
            <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider' }}>
              <Typography variant="h6">Query Results</Typography>
              {results?.executionTime && (
                <Typography variant="body2" color="text.secondary">
                  Execution time: {results.executionTime}ms
                </Typography>
              )}
            </Box>

            <Box sx={{ flexGrow: 1, overflow: 'auto', p: 2 }}>
              {error && (
                <Alert severity="error" sx={{ mb: 2 }}>
                  {error}
                </Alert>
              )}

              {!results ? (
                <Typography variant="body2" color="text.secondary">
                  No query executed yet. Enter a SQL query above and click 'Execute Query'.
                </Typography>
              ) : results.success && results.data ? (
                <TableContainer>
                  <Table size="small">
                    <TableHead>
                      <TableRow>
                        {results.columns?.map((column, index) => (
                          <TableCell key={index}>{column}</TableCell>
                        ))}
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {results.data.map((row, index) => (
                        <TableRow key={index}>
                          {results.columns?.map((column, colIndex) => (
                            <TableCell key={colIndex}>
                              {JSON.stringify(row[column])}
                            </TableCell>
                          ))}
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                </TableContainer>
              ) : (
                <Alert severity={results.success ? 'success' : 'error'}>
                  {results.message}
                </Alert>
              )}
            </Box>
          </Paper>
        </Box>
      </Box>
    </Box>
  );
};

export default SqlEditor;
