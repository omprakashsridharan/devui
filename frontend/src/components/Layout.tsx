import React, { useState, useEffect } from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import {
  AppBar,
  Toolbar,
  Typography,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Box,
  IconButton,
  Divider,
  CircularProgress,
  Chip,
} from '@mui/material';
import {
  Home as HomeIcon,
  Menu as MenuIcon,
  ChevronLeft as ChevronLeftIcon,
  Api as ApiIcon,
  Storage as StorageIcon,
  CloudQueue as KafkaIcon,
  Dataset as DatabaseIcon,
  Settings as SettingsIcon,
  Extension as ExtensionIcon,
} from '@mui/icons-material';
import { servicesService, type Service } from '../services/servicesService';

const DRAWER_WIDTH = 240;
const DRAWER_WIDTH_COLLAPSED = 60;

interface MenuItem {
  text: string;
  path: string;
  icon: React.ReactElement;
  type?: 'static' | 'service';
  serviceName?: string;
}

// Static menu items (always shown)
const staticMenuItems: MenuItem[] = [
  {
    text: 'Home',
    path: '/',
    icon: <HomeIcon />,
    type: 'static',
  },
];

// Service icon mapping
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

const Layout: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const [drawerOpen, setDrawerOpen] = useState(true);
  const [services, setServices] = useState<Service[]>([]);
  const [loadingServices, setLoadingServices] = useState(true);
  const [servicesError, setServicesError] = useState<string | null>(null);

  // Fetch services on component mount
  useEffect(() => {
    const fetchServices = async () => {
      try {
        setLoadingServices(true);
        setServicesError(null);
        const servicesData = await servicesService.getServices();
        setServices(servicesData);
      } catch (error) {
        console.error('Failed to load services:', error);
        setServicesError(error instanceof Error ? error.message : 'Failed to load services');
      } finally {
        setLoadingServices(false);
      }
    };

    fetchServices();
  }, []);

  // Create dynamic menu items from services
  const serviceMenuItems: MenuItem[] = services.map(service => {
    const menuItem = {
      text: service.name,
      path: `/service/${service.name.toLowerCase()}`,
      icon: getServiceIcon(service.name),
      type: 'service' as const,
      serviceName: service.name,
    };
    console.log('Layout: Created menu item:', menuItem);
    return menuItem;
  });

  // Combine static and service menu items
  const allMenuItems: MenuItem[] = [
    ...staticMenuItems,
    ...serviceMenuItems,
  ];

  const handleNavigation = (path: string) => {
    console.log('Layout: Navigating to path:', path);
    navigate(path);
  };

  const toggleDrawer = () => {
    setDrawerOpen(!drawerOpen);
  };

  const drawer = (
    <Box>
      <Toolbar sx={{
        justifyContent: drawerOpen ? 'space-between' : 'center',
        minHeight: 48,
        px: drawerOpen ? 2 : 0,
        backgroundColor: 'background.paper',
        color: 'text.primary',
        borderBottom: '1px solid',
        borderColor: 'divider',
      }}>
        {drawerOpen && (
          <Typography variant="h6" noWrap component="div" sx={{ fontWeight: 'bold' }}>
            DevUI
          </Typography>
        )}
        <IconButton
          onClick={toggleDrawer}
          sx={{
            ml: drawerOpen ? 0 : 'auto',
            mr: drawerOpen ? 0 : 'auto',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            color: 'text.primary',
            '&:hover': {
              backgroundColor: 'action.hover',
            },
          }}
        >
          {drawerOpen ? <ChevronLeftIcon /> : <MenuIcon />}
        </IconButton>
      </Toolbar>
      <List>
        {/* Static Menu Items */}
        {staticMenuItems.map((item) => (
          <ListItem key={item.text} disablePadding>
            <ListItemButton
              onClick={() => handleNavigation(item.path)}
              selected={location.pathname === item.path}
              sx={{
                minHeight: 48,
                justifyContent: drawerOpen ? 'initial' : 'center',
                px: drawerOpen ? 2.5 : 1,
                mx: 1,
                borderRadius: 1,
                '&.Mui-selected': {
                  backgroundColor: 'action.selected',
                  color: 'text.primary',
                  '&:hover': {
                    backgroundColor: 'action.selected',
                  },
                  '& .MuiListItemIcon-root': {
                    color: 'text.primary',
                  },
                },
                '&:hover': {
                  backgroundColor: 'action.hover',
                },
              }}
            >
              <ListItemIcon
                sx={{
                  minWidth: 0,
                  mr: drawerOpen ? 1 : 0,
                  justifyContent: 'center',
                  display: 'flex',
                  alignItems: 'center',
                }}
              >
                {item.icon}
              </ListItemIcon>
              {drawerOpen && <ListItemText primary={item.text} />}
            </ListItemButton>
          </ListItem>
        ))}

        {/* Services Section */}
        {drawerOpen && (
          <>
            <Divider sx={{ my: 1, mx: 2 }} />
            <Box sx={{ px: 2, py: 1 }}>
              <Typography variant="overline" color="text.secondary" sx={{ fontWeight: 'bold' }}>
                Services
              </Typography>
            </Box>
          </>
        )}

        {/* Services Loading */}
        {loadingServices && drawerOpen && (
          <ListItem disablePadding>
            <Box sx={{ display: 'flex', alignItems: 'center', px: 2.5, py: 1, mx: 1 }}>
              <CircularProgress size={20} sx={{ mr: 1 }} />
              <Typography variant="body2" color="text.secondary">
                Loading services...
              </Typography>
            </Box>
          </ListItem>
        )}

        {/* Services Error */}
        {servicesError && drawerOpen && (
          <ListItem disablePadding>
            <Box sx={{ px: 2.5, py: 1, mx: 1 }}>
              <Typography variant="body2" color="error">
                Failed to load services
              </Typography>
            </Box>
          </ListItem>
        )}

        {/* Dynamic Service Menu Items */}
        {serviceMenuItems.map((item) => {
          const service = services.find(s => s.name === item.serviceName);
          return (
            <ListItem key={item.text} disablePadding>
              <ListItemButton
                onClick={() => handleNavigation(item.path)}
                selected={(() => {
                  // Check if this is the active service based on current path
                  if (item.type === 'service' && item.serviceName) {
                    // For SQL service, check if we're on /sql route
                    if (item.serviceName.toLowerCase() === 'sql' && location.pathname === '/sql') {
                      return true;
                    }
                    // For Kafka service, check if we're on /kafka route
                    if (item.serviceName.toLowerCase() === 'kafka' && location.pathname === '/kafka') {
                      return true;
                    }
                    // For other services, check the service route
                    if (location.pathname === item.path) {
                      return true;
                    }
                  }
                  // For non-service items, check exact path match
                  return location.pathname === item.path;
                })()}
                sx={{
                  minHeight: 48,
                  justifyContent: drawerOpen ? 'initial' : 'center',
                  px: drawerOpen ? 2.5 : 1,
                  mx: 1,
                  borderRadius: 1,
                  '&.Mui-selected': {
                    backgroundColor: 'primary.main',
                    color: 'primary.contrastText',
                    boxShadow: 1,
                    '&:hover': {
                      backgroundColor: 'primary.dark',
                      boxShadow: 2,
                    },
                    '& .MuiListItemIcon-root': {
                      color: 'primary.contrastText',
                    },
                  },
                  '&:hover': {
                    backgroundColor: 'action.hover',
                    boxShadow: 1,
                  },
                }}
              >
                <ListItemIcon
                  sx={{
                    minWidth: 0,
                    mr: drawerOpen ? 1 : 0,
                    justifyContent: 'center',
                    display: 'flex',
                    alignItems: 'center',
                  }}
                >
                  {item.icon}
                </ListItemIcon>
                {drawerOpen && (
                  <ListItemText
                    primary={item.text}
                    secondary={
                      service && (
                        <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5, mt: 0.5 }}>
                          <Chip
                            label={service.available ? 'Available' : 'Unavailable'}
                            size="small"
                            variant="filled"
                            color={service.available ? 'success' : 'error'}
                            sx={{ fontSize: '0.7rem', height: 20 }}
                          />
                        </Box>
                      )
                    }
                  />
                )}
              </ListItemButton>
            </ListItem>
          );
        })}
      </List>
    </Box>
  );

  const currentDrawerWidth = drawerOpen ? DRAWER_WIDTH : DRAWER_WIDTH_COLLAPSED;

  // Helper function to find the current menu item based on pathname
  const getCurrentMenuItem = () => {
    // First check for exact path matches
    let currentItem = allMenuItems.find(item => item.path === location.pathname);

    // If no exact match and we're on a service route, find the corresponding service
    if (!currentItem) {
      if (location.pathname === '/sql') {
        currentItem = allMenuItems.find(item => item.serviceName?.toLowerCase() === 'sql');
      } else if (location.pathname === '/kafka') {
        currentItem = allMenuItems.find(item => item.serviceName?.toLowerCase() === 'kafka');
      }
    }

    return currentItem;
  };

  return (
    <Box sx={{ display: 'flex', minHeight: '100vh' }}>
      <AppBar
        position="fixed"
        sx={{
          width: `calc(100% - ${currentDrawerWidth}px)`,
          ml: `${currentDrawerWidth}px`,
          border: 'none',
          boxShadow: 'none',
          backgroundColor: 'background.paper',
          borderBottom: '1px solid',
          borderColor: 'divider',
          transition: (theme) =>
            theme.transitions.create(['width', 'margin', 'backgroundColor'], {
              easing: theme.transitions.easing.sharp,
              duration: theme.transitions.duration.leavingScreen,
            }),
        }}
      >
        <Toolbar>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
            <Typography variant="h6" noWrap component="div">
              {getCurrentMenuItem()?.text || 'DevUI'}
            </Typography>

            {/* Active Service Indicator */}
            {(() => {
              const currentItem = getCurrentMenuItem();
              if (currentItem && currentItem.type === 'service') {
                const service = services.find(s => s.name === currentItem.serviceName);
                return (
                  <Box sx={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: 1,
                    px: 2,
                    py: 0.5,
                    backgroundColor: 'action.hover',
                    borderRadius: 1,
                    border: '1px solid',
                    borderColor: 'divider',
                  }}>
                    <Box sx={{ display: 'flex', alignItems: 'center' }}>
                      {currentItem.icon}
                    </Box>
                    <Chip
                      label={service?.available ? 'Available' : 'Unavailable'}
                      size="small"
                      variant="filled"
                      color={service?.available ? 'success' : 'error'}
                      sx={{ fontSize: '0.75rem', height: 24 }}
                    />
                  </Box>
                );
              }
              return null;
            })()}
          </Box>
        </Toolbar>
      </AppBar>

      <Drawer
        variant="permanent"
        sx={{
          width: currentDrawerWidth,
          flexShrink: 0,
          '& .MuiDrawer-paper': {
            width: currentDrawerWidth,
            boxSizing: 'border-box',
            backgroundColor: 'background.paper',
            border: 'none',
            borderRight: '1px solid',
            borderRightColor: 'divider',
            borderLeft: 'none',
            borderTop: 'none',
            borderBottom: 'none',
            margin: 0,
            padding: 0,
            transition: (theme) =>
              theme.transitions.create('width', {
                easing: theme.transitions.easing.sharp,
                duration: theme.transitions.duration.enteringScreen,
              }),
            overflowX: 'hidden',
          },
        }}
      >
        {drawer}
      </Drawer>

      <Box
        component="main"
        sx={{
          flexGrow: 1,
          p: 3,
          width: `calc(100% - ${currentDrawerWidth}px)`,
          mt: '64px', // Height of AppBar
          border: 'none',
          transition: (theme) =>
            theme.transitions.create(['width', 'margin'], {
              easing: theme.transitions.easing.sharp,
              duration: theme.transitions.duration.leavingScreen,
            }),
        }}
      >
        <Outlet />
      </Box>
    </Box>
  );
};

export default Layout;
