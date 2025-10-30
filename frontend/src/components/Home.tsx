import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Box, Paper, Typography, Chip, CircularProgress, Alert } from '@mui/material';
import { servicesService, type Service } from '../services/servicesService';

const Home = () => {
  const navigate = useNavigate();
  const [services, setServices] = useState<Service[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const load = async () => {
      try {
        setLoading(true);
        setError(null);
        const data = await servicesService.getServices();
        setServices(data);
      } catch (e) {
        setError(e instanceof Error ? e.message : 'Failed to load services');
      } finally {
        setLoading(false);
      }
    };
    load();
  }, []);

  const getServicePath = (name: string) => {
    const lower = name.toLowerCase();
    if (lower === 'sql') return '/sql';
    if (lower === 'kafka') return '/kafka';
    return `/service/${lower}`;
  };

  return (
    <Box sx={{ p: 2 }}>
      <Typography variant="h4" gutterBottom>
        DevUI - Services
      </Typography>
      <Typography variant="body2" color="text.secondary" sx={{ mb: 2 }}>
        Select a service to get started.
      </Typography>

      {loading ? (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <CircularProgress size={20} />
          <Typography variant="body2">Loading services...</Typography>
        </Box>
      ) : error ? (
        <Alert severity="error">{error}</Alert>
      ) : (
        <Box sx={{
          display: 'grid',
          gridTemplateColumns: {
            xs: '1fr',
            sm: '1fr 1fr',
            md: '1fr 1fr 1fr',
            lg: '1fr 1fr 1fr 1fr',
          },
          gap: 2,
        }}>
          {services.map((svc) => (
            <Paper
              key={svc.name}
              onClick={() => navigate(getServicePath(svc.name))}
              sx={{ p: 2, cursor: 'pointer', height: '100%', display: 'flex', flexDirection: 'column',
                '&:hover': { backgroundColor: 'action.hover' },
                border: '1px solid', borderColor: 'divider' }}
            >
              <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
                <Typography variant="h6" component="h3">{svc.name}</Typography>
                <Chip
                  label={svc.available ? 'Available' : 'Unavailable'}
                  color={svc.available ? 'success' : 'error'}
                  size="small"
                  sx={{ fontWeight: 'bold' }}
                />
              </Box>
              <Typography variant="body2" color="text.secondary">
                {svc.name === 'SQL' && 'Query and manage databases'}
                {svc.name === 'Kafka' && 'Manage topics and messages'}
                {svc.name !== 'SQL' && svc.name !== 'Kafka' && 'Open service dashboard'}
              </Typography>
            </Paper>
          ))}
        </Box>
      )}
    </Box>
  );
};

export default Home;
