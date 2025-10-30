import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import {
  Box,
  Paper,
  Typography,
  Button,
  Alert,
  Divider,
  IconButton,
  Tooltip,
  Switch,
  FormControlLabel,
} from '@mui/material';
import {
  PlayArrow as StartIcon,
  Stop as StopIcon,
  ContentCopy as CopyIcon,
  DeleteSweep as ClearIcon,
  Download as DownloadIcon,
} from '@mui/icons-material';
import { getApiUrl } from '../config/api';

interface KafkaConsumeProps {
  clusterName: string;
  topicName: string;
}

type KafkaEvent = {
  raw: string;
  parsed?: any;
  receivedAt: string;
};

const KafkaConsume: React.FC<KafkaConsumeProps> = ({ clusterName, topicName }) => {
  const [isConsuming, setIsConsuming] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [events, setEvents] = useState<KafkaEvent[]>([]);
  const [autoScroll, setAutoScroll] = useState(true);
  const [prettyPrint, setPrettyPrint] = useState(true);
  const eventSourceRef = useRef<EventSource | null>(null);
  const endRef = useRef<HTMLDivElement | null>(null);

  const sseUrl = useMemo(() => {
    return getApiUrl(`/services/kafka/clusters/${clusterName}/topics/${topicName}/consume`);
  }, [clusterName, topicName]);

  const stopConsuming = useCallback(() => {
    if (eventSourceRef.current) {
      eventSourceRef.current.close();
      eventSourceRef.current = null;
    }
    setIsConsuming(false);
  }, []);

  const startConsuming = useCallback(() => {
    // Reset
    setErrorMessage(null);
    setIsConsuming(true);

    try {
      const es = new EventSource(sseUrl);
      eventSourceRef.current = es;

      es.onmessage = (e) => {
        const raw = e.data;
        let parsed: any | undefined = undefined;
        try {
          parsed = JSON.parse(raw);
        } catch (_) {
          // leave parsed undefined if not JSON
        }
        setEvents((prev) => [
          ...prev,
          {
            raw,
            parsed,
            receivedAt: new Date().toISOString(),
          },
        ]);
      };

      es.onerror = () => {
        setErrorMessage('Connection error while consuming.');
      };
    } catch (err) {
      setIsConsuming(false);
      setErrorMessage(err instanceof Error ? err.message : 'Failed to start consumer');
    }
  }, [sseUrl]);

  useEffect(() => {
    if (autoScroll && endRef.current) {
      endRef.current.scrollIntoView({ behavior: 'smooth', block: 'end' });
    }
  }, [events, autoScroll]);

  useEffect(() => {
    return () => {
      // Cleanup on unmount
      if (eventSourceRef.current) {
        eventSourceRef.current.close();
      }
    };
  }, []);

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const handleClear = () => setEvents([]);

  const handleDownload = () => {
    const blob = new Blob([events.map((e) => e.raw).join('\n')], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${clusterName}-${topicName}-events.txt`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <Paper sx={{ p: 2, height: '100%', display: 'flex', flexDirection: 'column' }}>
      <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 2 }}>
        <Typography variant="h6" component="h2">Consume Messages</Typography>
        <Typography variant="body2" color="text.secondary">{clusterName} / {topicName}</Typography>
      </Box>

      <Divider sx={{ mb: 2 }} />

      {errorMessage && (
        <Alert severity="error" sx={{ mb: 2 }} onClose={() => setErrorMessage(null)}>
          {errorMessage}
        </Alert>
      )}

      <Box sx={{ display: 'flex', gap: 1, flexWrap: 'wrap', mb: 2 }}>
        <Button
          variant="contained"
          color={isConsuming ? 'inherit' : 'primary'}
          startIcon={<StartIcon />}
          onClick={startConsuming}
          disabled={isConsuming}
        >
          Start Consuming
        </Button>
        <Button
          variant="outlined"
          color="error"
          startIcon={<StopIcon />}
          onClick={stopConsuming}
          disabled={!isConsuming}
        >
          Stop
        </Button>
        <Tooltip title="Copy all events">
          <span>
            <IconButton size="small" onClick={() => handleCopy(events.map((e) => e.raw).join('\n'))} disabled={events.length === 0}>
              <CopyIcon fontSize="small" />
            </IconButton>
          </span>
        </Tooltip>
        <Tooltip title="Download events">
          <span>
            <IconButton size="small" onClick={handleDownload} disabled={events.length === 0}>
              <DownloadIcon fontSize="small" />
            </IconButton>
          </span>
        </Tooltip>
        <Tooltip title="Clear events">
          <span>
            <IconButton size="small" onClick={handleClear} disabled={events.length === 0}>
              <ClearIcon fontSize="small" />
            </IconButton>
          </span>
        </Tooltip>
        <FormControlLabel
          sx={{ ml: 'auto' }}
          control={<Switch checked={autoScroll} onChange={(_, v) => setAutoScroll(v)} size="small" />}
          label={<Typography variant="caption">Auto-scroll</Typography>}
        />
        <FormControlLabel
          control={<Switch checked={prettyPrint} onChange={(_, v) => setPrettyPrint(v)} size="small" />}
          label={<Typography variant="caption">Pretty JSON</Typography>}
        />
      </Box>

      <Box sx={{ flex: 1, overflow: 'auto', bgcolor: 'grey.50', borderRadius: 1, p: 1 }}>
        {events.length === 0 ? (
          <Typography variant="body2" color="text.secondary" sx={{ p: 1 }}>
            No events yet. Click "Start Consuming" to begin streaming messages.
          </Typography>
        ) : (
          events.map((evt, idx) => (
            <Box key={idx} sx={{ mb: 1.5, p: 1, bgcolor: 'background.paper', border: '1px solid', borderColor: 'divider', borderRadius: 1 }}>
              <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 0.5 }}>
                <Typography variant="caption" color="text.secondary">{evt.receivedAt}</Typography>
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                  <Tooltip title="Copy">
                    <IconButton size="small" onClick={() => handleCopy(evt.raw)}>
                      <CopyIcon fontSize="inherit" />
                    </IconButton>
                  </Tooltip>
                </Box>
              </Box>
              <Box component="pre" sx={{ m: 0, whiteSpace: 'pre-wrap', wordBreak: 'break-word', fontFamily: 'monospace', fontSize: '0.8rem' }}>
                {prettyPrint && evt.parsed ? JSON.stringify(evt.parsed, null, 2) : evt.raw}
              </Box>
            </Box>
          ))
        )}
        <div ref={endRef} />
      </Box>
    </Paper>
  );
};

export default KafkaConsume;


