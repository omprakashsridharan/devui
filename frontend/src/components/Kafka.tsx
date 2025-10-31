import React, {useState, useEffect} from 'react';
import {
    Box,
    Typography,
    Paper,
    CircularProgress,
    Alert,
    List,
    ListItem,
    ListItemButton,
    ListItemText,
    ListItemIcon,
    Divider,
    Collapse,
    IconButton,
    Card,
    CardContent,
    Grid,
} from '@mui/material';
import {
    Storage as BrokerIcon,
    Topic as TopicIcon,
    ExpandMore as ExpandMoreIcon,
    ExpandLess as ExpandLessIcon,
} from '@mui/icons-material';
import {kafkaService, type KafkaClusterMetadata,} from '../services/kafkaService';
import KafkaProduce from './KafkaProduce';
import KafkaConsume from './KafkaConsume';

const Kafka: React.FC = () => {
    const [clusters, setClusters] = useState<string[]>([]);
    const [selectedCluster, setSelectedCluster] = useState<string>('');
    const [clusterMetadata, setClusterMetadata] = useState<KafkaClusterMetadata | null>(null);
    const [loadingClusters, setLoadingClusters] = useState(true);
    const [loadingMetadata, setLoadingMetadata] = useState(false);
    const [clustersError, setClustersError] = useState<string | null>(null);
    const [metadataError, setMetadataError] = useState<string | null>(null);
    const [brokersExpanded, setBrokersExpanded] = useState(false);
    const [selectedTopic, setSelectedTopic] = useState<string>('');
    const [consumerKey, setConsumerKey] = useState<string>(''); // Force consumer remount when topic changes

    // Fetch clusters on component mount
    useEffect(() => {
        const fetchClusters = async () => {
            try {
                setLoadingClusters(true);
                setClustersError(null);
                const clustersData = await kafkaService.getClusters();
                setClusters(clustersData);
                if (clustersData.length > 0) {
                    setSelectedCluster(clustersData[0]);
                }
            } catch (error) {
                console.error('Failed to load clusters:', error);
                setClustersError(error instanceof Error ? error.message : 'Failed to load clusters');
            } finally {
                setLoadingClusters(false);
            }
        };

        fetchClusters();
    }, []);

    // Fetch cluster metadata when selected cluster changes
    useEffect(() => {
        const fetchClusterMetadata = async () => {
            if (!selectedCluster) return;

            try {
                setLoadingMetadata(true);
                setMetadataError(null);
                const metadata = await kafkaService.getClusterMetadata(selectedCluster);
                setClusterMetadata(metadata);
            } catch (error) {
                console.error('Failed to load cluster metadata:', error);
                setMetadataError(error instanceof Error ? error.message : 'Failed to load cluster metadata');
                setClusterMetadata(null);
            } finally {
                setLoadingMetadata(false);
            }
        };

        fetchClusterMetadata();
    }, [selectedCluster]);


    const handleBrokersToggle = () => {
        setBrokersExpanded(!brokersExpanded);
    };

    const handleTopicSelect = (topicName: string) => {
        if (selectedTopic !== topicName) {
            setSelectedTopic(topicName);
            // Force consumer to remount by changing key, which will stop any active consumer
            setConsumerKey(`${selectedCluster}-${topicName}-${Date.now()}`);
        }
    };

    if (loadingClusters) {
        return (
            <Box sx={{p: 2}}>
                <Box sx={{display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: 150}}>
                    <CircularProgress/>
                    <Typography variant="body2" sx={{ml: 2}}>
                        Loading Kafka clusters...
                    </Typography>
                </Box>
            </Box>
        );
    }

    if (clustersError) {
        return (
            <Box sx={{p: 2}}>
                <Alert severity="error" sx={{m: 1}}>
                    <Typography variant="h6" sx={{mb: 1}}>
                        Failed to load Kafka clusters
                    </Typography>
                    <Typography variant="body2">
                        {clustersError}
                    </Typography>
                </Alert>
            </Box>
        );
    }

    if (clusters.length === 0) {
        return (
            <Box sx={{p: 2}}>
                <Alert severity="info" sx={{m: 1}}>
                    <Typography variant="h6" sx={{mb: 1}}>
                        No Kafka clusters found
                    </Typography>
                    <Typography variant="body2">
                        No Kafka clusters are currently available.
                    </Typography>
                </Alert>
            </Box>
        );
    }

    return (
        <Box sx={{p: 1, height: 'calc(100vh - 120px)'}}>
            <Box sx={{display: 'flex', gap: 2, height: '100%'}}>
                {/* Left Panel - Cluster and Topics */}
                <Box sx={{flex: selectedTopic ? '0 0 30%' : '1', transition: 'flex 0.3s ease'}}>
                    {/* Cluster Tabs */}
                    <Paper sx={{height: '100%', display: 'flex', flexDirection: 'column'}}>
                        <Box sx={{borderBottom: 1, borderColor: 'divider'}}>
                            <Box sx={{display: 'flex', overflowX: 'auto'}}>
                                {clusters.map((cluster) => (
                                    <Box
                                        key={cluster}
                                        onClick={() => setSelectedCluster(cluster)}
                                        sx={{
                                            px: 2,
                                            py: 1,
                                            cursor: 'pointer',
                                            borderBottom: selectedCluster === cluster ? 2 : 0,
                                            borderBottomColor: 'primary.main',
                                            backgroundColor: selectedCluster === cluster ? 'action.selected' : 'transparent',
                                            fontWeight: selectedCluster === cluster ? 'bold' : 'normal',
                                            minWidth: 'fit-content',
                                            whiteSpace: 'nowrap',
                                        }}
                                    >
                                        <Typography variant="body2" sx={{textTransform: 'none'}}>
                                            {cluster}
                                        </Typography>
                                    </Box>
                                ))}
                            </Box>
                        </Box>

                        {/* Cluster Content */}
                        <Box sx={{flex: 1, overflow: 'auto'}}>
                            {loadingMetadata ? (
                                <Box sx={{
                                    display: 'flex',
                                    justifyContent: 'center',
                                    alignItems: 'center',
                                    minHeight: 100,
                                    p: 2
                                }}>
                                    <CircularProgress/>
                                    <Typography variant="body2" sx={{ml: 2}}>
                                        Loading cluster metadata...
                                    </Typography>
                                </Box>
                            ) : metadataError ? (
                                <Alert severity="error" sx={{m: 1}}>
                                    <Typography variant="h6" sx={{mb: 1}}>
                                        Failed to load cluster metadata
                                    </Typography>
                                    <Typography variant="body2">
                                        {metadataError}
                                    </Typography>
                                </Alert>
                            ) : clusterMetadata ? (
                                <Box>
                                    {/* Brokers Expandable Section */}
                                    <ListItem
                                        sx={{
                                            borderBottom: 1,
                                            borderColor: 'divider',
                                            backgroundColor: 'action.hover',
                                            p: 0,
                                        }}
                                    >
                                        <ListItemButton onClick={handleBrokersToggle}>
                                            <ListItemIcon>
                                                <BrokerIcon color="primary"/>
                                            </ListItemIcon>
                                            <ListItemText
                                                primary={`Brokers (${Object.keys(clusterMetadata.brokers).length})`}
                                                primaryTypographyProps={{fontWeight: 'bold'}}
                                            />
                                            <IconButton size="small">
                                                {brokersExpanded ? <ExpandLessIcon/> : <ExpandMoreIcon/>}
                                            </IconButton>
                                        </ListItemButton>
                                    </ListItem>

                                    <Collapse in={brokersExpanded} timeout="auto" unmountOnExit>
                                        <Box sx={{p: 1, backgroundColor: 'grey.50'}}>
                                            <Grid container spacing={1}>
                                                {Object.entries(clusterMetadata.brokers).map(([_, broker]) => (
                                                    <Grid size={{ xs: 12, sm: 6, md: 4 }} key={broker.id}>
                                                        <Card sx={{height: '100%'}}>
                                                            <CardContent sx={{p: 1}}>
                                                                <Box sx={{
                                                                    display: 'flex',
                                                                    alignItems: 'center',
                                                                    mb: 0.5
                                                                }}>
                                                                    <BrokerIcon
                                                                        sx={{
                                                                            mr: 0.5,
                                                                            color: 'primary.main',
                                                                            fontSize: 16
                                                                        }}/>
                                                                    <Typography variant="subtitle2"
                                                                                sx={{
                                                                                    fontWeight: 'bold',
                                                                                    fontSize: '0.8rem'
                                                                                }}>
                                                                        Broker {broker.id}
                                                                    </Typography>
                                                                </Box>
                                                                <Typography variant="body2" color="text.secondary"
                                                                            sx={{fontSize: '0.7rem'}}>
                                                                    Host: {broker.host}
                                                                </Typography>
                                                                <Typography variant="body2" color="text.secondary"
                                                                            sx={{fontSize: '0.7rem'}}>
                                                                    Port: {broker.port}
                                                                </Typography>
                                                                <Typography variant="body2" color="text.secondary"
                                                                            sx={{fontSize: '0.7rem'}}>
                                                                    Address: {broker.host}:{broker.port}
                                                                </Typography>
                                                            </CardContent>
                                                        </Card>
                                                    </Grid>
                                                ))}
                                            </Grid>
                                        </Box>
                                    </Collapse>

                                    {/* Topics List */}
                                    <Box>
                                        <ListItem
                                            sx={{
                                                borderBottom: 1,
                                                borderColor: 'divider',
                                                backgroundColor: 'action.hover'
                                            }}>
                                            <ListItemIcon>
                                                <TopicIcon color="primary"/>
                                            </ListItemIcon>
                                            <ListItemText
                                                primary={`Topics (${clusterMetadata.topics.length})`}
                                                primaryTypographyProps={{fontWeight: 'bold'}}
                                            />
                                        </ListItem>

                                        <Box sx={{flex: 1, overflow: 'auto'}}>
                                            <List dense>
                                                {clusterMetadata.topics.map((topic, index) => (
                                                    <React.Fragment key={topic.name}>
                                                        <ListItem
                                                            sx={{
                                                                py: 0.5,
                                                                px: 2,
                                                                cursor: 'pointer',
                                                                backgroundColor: selectedTopic === topic.name ? 'primary.main' : 'transparent',
                                                                color: selectedTopic === topic.name ? 'primary.contrastText' : 'inherit',
                                                                borderLeft: selectedTopic === topic.name ? 4 : 0,
                                                                borderLeftColor: 'primary.dark',
                                                                '&:hover': {
                                                                    backgroundColor: selectedTopic === topic.name ? 'primary.dark' : 'action.hover',
                                                                },
                                                                transition: 'all 0.2s ease',
                                                            }}
                                                            onClick={() => handleTopicSelect(topic.name)}
                                                        >
                                                            <ListItemIcon sx={{minWidth: 32}}>
                                                                <TopicIcon
                                                                    color={selectedTopic === topic.name ? 'inherit' : 'primary'}
                                                                    sx={{fontSize: 18}}
                                                                />
                                                            </ListItemIcon>
                                                            <ListItemText
                                                                primary={topic.name}
                                                                secondary={`${topic.partitions.length} partition(s)`}
                                                                primaryTypographyProps={{
                                                                    fontSize: '0.85rem',
                                                                    fontWeight: selectedTopic === topic.name ? 'bold' : 'normal'
                                                                }}
                                                                secondaryTypographyProps={{
                                                                    fontSize: '0.7rem',
                                                                    color: selectedTopic === topic.name ? 'rgba(255,255,255,0.7)' : 'text.secondary'
                                                                }}
                                                            />
                                                        </ListItem>
                                                        {index < clusterMetadata.topics.length - 1 && <Divider/>}
                                                    </React.Fragment>
                                                ))}
                                            </List>
                                        </Box>
                                    </Box>
                                </Box>
                            ) : null}
                        </Box>
                    </Paper>
                </Box>

                {/* Right Panel - Producer and Consumer Side by Side */}
                {selectedTopic && (
                    <Box sx={{flex: '1', transition: 'flex 0.3s ease', display: 'flex', flexDirection: 'column', minHeight: 0}}>
                        {/* Active Topic Header */}
                        <Paper sx={{ p: 2, mb: 2, bgcolor: 'primary.main', color: 'primary.contrastText' }}>
                            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                                <TopicIcon />
                                <Typography variant="h6" component="h2">
                                  Active Topic: {selectedTopic}
                                </Typography>
                                <Typography variant="body2" sx={{ ml: 'auto', opacity: 0.8 }}>
                                  Cluster: {selectedCluster}
                                </Typography>
                            </Box>
                        </Paper>

                        {/* Producer and Consumer Panels */}
                        <Box sx={{ display: 'flex', gap: 2, flex: 1, minHeight: 0 }}>
                            {/* Producer */}
                            <Box sx={{flex: '1', minHeight: 0}}>
                                <KafkaProduce
                                    clusterName={selectedCluster}
                                    topicName={selectedTopic}
                                />
                            </Box>

                            {/* Consumer */}
                            <Box sx={{flex: '1', minHeight: 0}}>
                                <KafkaConsume
                                    key={consumerKey}
                                    clusterName={selectedCluster}
                                    topicName={selectedTopic}
                                />
                            </Box>
                        </Box>
                    </Box>
                )}
            </Box>
        </Box>
    );
};

export default Kafka;
