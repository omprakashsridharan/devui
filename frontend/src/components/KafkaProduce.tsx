import React, { useState } from 'react';
import {
  Box,
  Paper,
  Typography,
  TextField,
  Button,
  Alert,
  CircularProgress,
  Divider,
  IconButton,
  Tooltip,
} from '@mui/material';
import {
  Send as SendIcon,
  ContentCopy as CopyIcon,
  Refresh as RefreshIcon,
} from '@mui/icons-material';
import { kafkaService, type KafkaProduceRequest } from '../services/kafkaService';

interface KafkaProduceProps {
  clusterName: string;
  topicName: string;
}

const KafkaProduce: React.FC<KafkaProduceProps> = ({ clusterName, topicName }) => {
  const [key, setKey] = useState('');
  const [value, setValue] = useState('');
  const [isProducing, setIsProducing] = useState(false);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const handleProduce = async () => {
    if (!key.trim() || !value.trim()) {
      setErrorMessage('Both key and value are required');
      return;
    }

    // Validate JSON value
    try {
      JSON.parse(value);
    } catch (_) {
      setErrorMessage('Value must be valid JSON');
      return;
    }

    setIsProducing(true);
    setSuccessMessage(null);
    setErrorMessage(null);

    try {
      const request: KafkaProduceRequest = {
        key: key.trim(),
        value: value.trim(),
      };

      await kafkaService.produceMessage(clusterName, topicName, request);
      setSuccessMessage('Message produced successfully!');

      // Clear form after successful produce
      setKey('');
      setValue('');
    } catch (error) {
      setErrorMessage(
        error instanceof Error ? error.message : 'Failed to produce message'
      );
    } finally {
      setIsProducing(false);
    }
  };

  const handleClearMessages = () => {
    setSuccessMessage(null);
    setErrorMessage(null);
  };

  const handleCopyValue = () => {
    navigator.clipboard.writeText(value);
  };

  const handleFormatJson = () => {
    try {
      const parsed = JSON.parse(value);
      setValue(JSON.stringify(parsed, null, 2));
    } catch (_) {
      setErrorMessage('Value must be valid JSON to format');
    }
  };

  const handleKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === 'Enter' && (event.ctrlKey || event.metaKey)) {
      handleProduce();
    }
  };

  return (
    <Paper sx={{ p: 2, height: '100%' }}>
      <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 2 }}>
        <Typography variant="h6" component="h2">
          Produce Message
        </Typography>
        <Typography variant="body2" color="text.secondary">
          {clusterName} / {topicName}
        </Typography>
      </Box>

      <Divider sx={{ mb: 2 }} />

      {/* Key Input */}
      <Box sx={{ mb: 2 }}>
        <Typography variant="subtitle2" sx={{ mb: 1, fontWeight: 'bold' }}>
          Key
        </Typography>
        <TextField
          fullWidth
          variant="outlined"
          size="small"
          placeholder="Enter message key..."
          value={key}
          onChange={(e) => setKey(e.target.value)}
          onKeyDown={handleKeyDown}
          disabled={isProducing}
        />
      </Box>

      {/* Value Input */}
      <Box sx={{ mb: 2 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
          <Typography variant="subtitle2" sx={{ fontWeight: 'bold' }}>
            Value (JSON)
          </Typography>
          <Box sx={{ display: 'flex', gap: 0.5 }}>
            <Tooltip title="Format JSON">
              <IconButton size="small" onClick={handleFormatJson} disabled={isProducing}>
                <RefreshIcon fontSize="small" />
              </IconButton>
            </Tooltip>
            <Tooltip title="Copy to clipboard">
              <IconButton size="small" onClick={handleCopyValue} disabled={isProducing || !value}>
                <CopyIcon fontSize="small" />
              </IconButton>
            </Tooltip>
          </Box>
        </Box>
        <TextField
          fullWidth
          multiline
          rows={8}
          variant="outlined"
          size="small"
          placeholder='Enter JSON value, e.g., {"message": "Hello World", "timestamp": "2024-01-01T00:00:00Z"}'
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onKeyDown={handleKeyDown}
          disabled={isProducing}
          sx={{
            '& .MuiInputBase-input': {
              fontFamily: 'monospace',
              fontSize: '0.875rem',
            },
          }}
        />
      </Box>

      {/* Action Buttons */}
      <Box sx={{ display: 'flex', gap: 1, mb: 2 }}>
        <Button
          variant="contained"
          startIcon={isProducing ? <CircularProgress size={16} /> : <SendIcon />}
          onClick={handleProduce}
          disabled={isProducing || !key.trim() || !value.trim()}
          sx={{ flex: 1 }}
        >
          {isProducing ? 'Producing...' : 'Produce Message'}
        </Button>
        <Button
          variant="outlined"
          onClick={handleClearMessages}
          disabled={isProducing}
        >
          Clear Messages
        </Button>
      </Box>

      {/* Success Message */}
      {successMessage && (
        <Alert
          severity="success"
          sx={{ mb: 1 }}
          onClose={handleClearMessages}
        >
          {successMessage}
        </Alert>
      )}

      {/* Error Message */}
      {errorMessage && (
        <Alert
          severity="error"
          sx={{ mb: 1 }}
          onClose={handleClearMessages}
        >
          {errorMessage}
        </Alert>
      )}

      {/* Help Text */}
      <Typography variant="caption" color="text.secondary" sx={{ display: 'block', mt: 1 }}>
        Tip: Use Ctrl+Enter (or Cmd+Enter on Mac) to quickly produce a message
      </Typography>
    </Paper>
  );
};

export default KafkaProduce;
