import React from 'react';
import {
  Box,
  Typography,
  Paper,
  Card,
  CardContent,
  Chip,
} from '@mui/material';
import {
  CloudQueue as KafkaIcon,
} from '@mui/icons-material';

const Kafka: React.FC = () => {
  return (
    <Box sx={{ p: 3 }}>
      <Paper sx={{ p: 3, mb: 3 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
          <Box sx={{ mr: 2, display: 'flex', alignItems: 'center' }}>
            <KafkaIcon sx={{ fontSize: 40, color: 'primary.main' }} />
          </Box>
          <Box sx={{ flexGrow: 1 }}>
            <Typography variant="h4" component="h1" sx={{ mb: 1 }}>
              Kafka Service
            </Typography>
            <Box sx={{ display: 'flex', gap: 1, alignItems: 'center' }}>
              <Chip
                label="Available"
                color="success"
                variant="filled"
              />
              <Chip
                label="Coming Soon"
                color="warning"
                variant="outlined"
              />
            </Box>
          </Box>
        </Box>

        <Typography variant="body1" color="text.secondary" sx={{ mb: 2 }}>
          Kafka messaging service management interface is currently under development.
        </Typography>
      </Paper>

      <Card>
        <CardContent>
          <Typography variant="h6" sx={{ mb: 2, display: 'flex', alignItems: 'center' }}>
            <KafkaIcon sx={{ mr: 1 }} />
            Coming Soon Features
          </Typography>
          <Box sx={{ pl: 2 }}>
            <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
              • Topic Management - Create, configure, and manage Kafka topics
            </Typography>
            <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
              • Message Monitoring - View and analyze message flows
            </Typography>
            <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
              • Consumer Groups - Monitor consumer group status and lag
            </Typography>
            <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
              • Producer Tools - Send test messages to topics
            </Typography>
            <Typography variant="body2" color="text.secondary" sx={{ mb: 1 }}>
              • Cluster Health - Monitor Kafka cluster performance
            </Typography>
            <Typography variant="body2" color="text.secondary">
              • Configuration Management - Manage Kafka configurations
            </Typography>
          </Box>
        </CardContent>
      </Card>
    </Box>
  );
};

export default Kafka;
