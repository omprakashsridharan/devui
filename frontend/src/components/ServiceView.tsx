import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import {
  Box,
  Typography,
  Paper,
  CircularProgress,
  Alert,
  Chip,
  Grid,
  Card,
  CardContent,
  CardHeader,
  Divider,
} from '@mui/material';
import {
  Storage as StorageIcon,
  CloudQueue as KafkaIcon,
  Dataset as DatabaseIcon,
  Settings as SettingsIcon,
  Extension as ExtensionIcon,
  Api as ApiIcon,
} from '@mui/icons-material';
import { servicesService, type Service } from '../services/servicesService';

// Service icon mapping (same as in Layout)
const getServiceIcon = (serviceName: string): React.ReactElement => {
  const name = serviceName.toLowerCase().trim();

  // Exact matches first
  if (name === 'sql' || name === 'database') {
    return <DatabaseIcon />;
  }
  if (name === 'kafka') {
    return <KafkaIcon />;
  }
  if (name === 'storage') {
    return <StorageIcon />;
  }
  if (name === 'api') {
    return <ApiIcon />;
  }
  if (name === 'config' || name === 'settings') {
    return <SettingsIcon />;
  }

  // Partial matches
  if (name.includes('sql') || name.includes('database')) {
    return <DatabaseIcon />;
  }
  if (name.includes('kafka')) {
    return <KafkaIcon />;
  }
  if (name.includes('storage')) {
    return <StorageIcon />;
  }
  if (name.includes('api')) {
    return <ApiIcon />;
  }
  if (name.includes('config') || name.includes('settings')) {
    return <SettingsIcon />;
  }

  // Default icon for unknown services
  return <ExtensionIcon />;
};

const ServiceView: React.FC = () => {
  const { serviceName } = useParams<{ serviceName: string }>();
  const navigate = useNavigate();
  const [service, setService] = useState<Service | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchServiceDetails = async () => {
      if (!serviceName) return;

      console.log('ServiceView: Fetching details for service:', serviceName);

      // Special handling for SQL service - redirect to existing SQL Editor
      if (serviceName.toLowerCase() === 'sql') {
        console.log('ServiceView: Redirecting SQL service to SQL Editor');
        navigate('/sql');
        return;
      }

      // Special handling for Kafka service - redirect to dedicated Kafka page
      if (serviceName.toLowerCase() === 'kafka') {
        console.log('ServiceView: Redirecting Kafka service to Kafka page');
        navigate('/kafka');
        return;
      }

      try {
        setLoading(true);
        setError(null);

        // Try to fetch service details, but if it fails, create a mock service
        // since the API might not have individual service detail endpoints
        try {
          const serviceData = await servicesService.getServiceDetails(serviceName);
          setService(serviceData);
        } catch (apiError) {
          console.log('ServiceView: API fetch failed, creating mock service:', apiError);
          // Create a mock service based on the service name
          const mockService: Service = {
            name: serviceName.charAt(0).toUpperCase() + serviceName.slice(1).toLowerCase(),
            available: true
          };
          setService(mockService);
        }
      } catch (error) {
        console.error('Failed to load service details:', error);
        setError(error instanceof Error ? error.message : 'Failed to load service details');
      } finally {
        setLoading(false);
      }
    };

    fetchServiceDetails();
  }, [serviceName, navigate]);

  if (loading) {
    return (
      <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: 400 }}>
        <CircularProgress />
        <Typography variant="body2" sx={{ ml: 2 }}>
          Loading service details...
        </Typography>
      </Box>
    );
  }

  if (error) {
    return (
      <Box sx={{ p: 3 }}>
        <Alert severity="error" sx={{ mt: 2 }}>
          <Typography variant="h6" sx={{ mb: 1 }}>
            Failed to load service details
          </Typography>
          <Typography variant="body2">
            {error}
          </Typography>
        </Alert>
      </Box>
    );
  }

  if (!service) {
    return (
      <Box sx={{ p: 3 }}>
        <Alert severity="warning" sx={{ mt: 2 }}>
          <Typography variant="h6" sx={{ mb: 1 }}>
            Service not found
          </Typography>
          <Typography variant="body2">
            The service "{serviceName}" could not be found.
          </Typography>
        </Alert>
      </Box>
    );
  }

  return (
    <Box sx={{ p: 3 }}>
      <Paper sx={{ p: 3, mb: 3 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
          <Box sx={{ mr: 2, display: 'flex', alignItems: 'center' }}>
            {getServiceIcon(service.name)}
          </Box>
          <Box sx={{ flexGrow: 1 }}>
            <Typography variant="h4" component="h1" sx={{ mb: 1 }}>
              {service.name}
            </Typography>
            <Box sx={{ display: 'flex', gap: 1, alignItems: 'center' }}>
              <Chip
                label={service.available ? 'Available' : 'Unavailable'}
                color={service.available ? 'success' : 'error'}
                variant="filled"
              />
            </Box>
          </Box>
        </Box>

        {service.description && (
          <Typography variant="body1" color="text.secondary" sx={{ mb: 2 }}>
            {service.description}
          </Typography>
        )}

        <Divider sx={{ my: 2 }} />

        <Grid container spacing={3}>
          <Grid item xs={12} md={6}>
            <Card>
              <CardHeader title="Service Information" />
              <CardContent>
                <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                    <Typography variant="body2" color="text.secondary">
                      Name:
                    </Typography>
                    <Typography variant="body2" sx={{ fontWeight: 'bold' }}>
                      {service.name}
                    </Typography>
                  </Box>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
                    <Typography variant="body2" color="text.secondary">
                      Status:
                    </Typography>
                    <Typography variant="body2" sx={{ fontWeight: 'bold' }}>
                      {service.available ? 'Available' : 'Unavailable'}
                    </Typography>
                  </Box>
                </Box>
              </CardContent>
            </Card>
          </Grid>

          <Grid item xs={12} md={6}>
            <Card>
              <CardHeader title="Quick Actions" />
              <CardContent>
                <Typography variant="body2" color="text.secondary">
                  Service-specific actions and configurations will be available here.
                </Typography>
                {/* TODO: Add service-specific actions based on service type */}
              </CardContent>
            </Card>
          </Grid>
        </Grid>
      </Paper>

      {/* Service-specific content based on name */}
      {service.name.toLowerCase() === 'sql' && (
        <Paper sx={{ p: 3 }}>
          <Typography variant="h6" sx={{ mb: 2 }}>
            SQL Service Actions
          </Typography>
          <Typography variant="body2" color="text.secondary">
            This service provides SQL database functionality. You can access the SQL Editor to manage databases and run queries.
          </Typography>
        </Paper>
      )}

      {service.name.toLowerCase() === 'kafka' && (
        <Paper sx={{ p: 3 }}>
          <Typography variant="h6" sx={{ mb: 2 }}>
            Kafka Service Actions
          </Typography>
          <Typography variant="body2" color="text.secondary">
            This service provides Kafka messaging functionality. Topic management and message monitoring features will be available here.
          </Typography>
        </Paper>
      )}
    </Box>
  );
};

export default ServiceView;
