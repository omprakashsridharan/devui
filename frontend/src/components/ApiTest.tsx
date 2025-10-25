import React, { useState } from 'react';
import { Button, Box, Typography, Alert, CircularProgress } from '@mui/material';
import { API_CONFIG, ENV_INFO } from '../config/api';
import { exampleService } from '../services/exampleService';

const ApiTest: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<string>('');
  const [error, setError] = useState<string>('');

  const testApiCall = async () => {
    setLoading(true);
    setError('');
    setResult('');

    try {
      // This will make a call to http://localhost:3000/api/data in development
      // or /api/data in production
      const data = await exampleService.fetchData();
      setResult(JSON.stringify(data, null, 2));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Box sx={{ p: 3 }}>
      <Typography variant="h5" gutterBottom>
        API Configuration Test
      </Typography>

      <Box sx={{ mb: 3 }}>
        <Typography variant="h6" gutterBottom>
          Current Configuration:
        </Typography>
        <Typography variant="body2" color="text.secondary">
          <strong>Environment:</strong> {ENV_INFO.mode}
        </Typography>
        <Typography variant="body2" color="text.secondary">
          <strong>Base URL:</strong> {API_CONFIG.BASE_URL || '(empty - relative URLs)'}
        </Typography>
        <Typography variant="body2" color="text.secondary">
          <strong>Development Mode:</strong> {ENV_INFO.isDevelopment ? 'Yes' : 'No'}
        </Typography>
        <Typography variant="body2" color="text.secondary">
          <strong>Production Mode:</strong> {ENV_INFO.isProduction ? 'Yes' : 'No'}
        </Typography>
      </Box>

      <Button
        variant="contained"
        onClick={testApiCall}
        disabled={loading}
        sx={{ mb: 2 }}
      >
        {loading ? <CircularProgress size={20} /> : 'Test API Call'}
      </Button>

      {error && (
        <Alert severity="error" sx={{ mb: 2 }}>
          {error}
        </Alert>
      )}

      {result && (
        <Box>
          <Typography variant="h6" gutterBottom>
            API Response:
          </Typography>
          <Box
            component="pre"
            sx={{
              backgroundColor: 'grey.100',
              p: 2,
              borderRadius: 1,
              overflow: 'auto',
              maxHeight: 300,
            }}
          >
            {result}
          </Box>
        </Box>
      )}
    </Box>
  );
};

export default ApiTest;
