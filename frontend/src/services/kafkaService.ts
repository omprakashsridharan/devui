import { api } from '../utils/apiClient';

export interface KafkaBroker {
  id: number;
  host: string;
  port: number;
}

export interface KafkaBrokers {
  [key: string]: KafkaBroker;
}

export interface KafkaPartition {
  partition: number;
  leader_id: number;
}

export interface KafkaTopic {
  name: string;
  partitions: KafkaPartition[];
}

export interface KafkaClusterMetadata {
  brokers: KafkaBrokers;
  topics: KafkaTopic[];
}

export interface KafkaProduceRequest {
  key: string;
  value: string; // Stringified JSON
}

/**
 * Service for managing Kafka operations
 */
export const kafkaService = {
  /**
   * Get list of Kafka clusters
   */
  async getClusters(): Promise<string[]> {
    try {
      const response = await api.get<string[]>('/services/kafka/clusters');
      return response || [];
    } catch (error) {
      console.error('Failed to fetch Kafka clusters:', error);
      throw new Error('Failed to fetch Kafka clusters');
    }
  },

  /**
   * Get cluster metadata including brokers and topics
   */
  async getClusterMetadata(clusterName: string): Promise<KafkaClusterMetadata> {
    try {
      const response = await api.get<KafkaClusterMetadata>(`/services/kafka/clusters/${clusterName}/metadata`);
      return response;
    } catch (error) {
      console.error(`Failed to fetch cluster metadata for ${clusterName}:`, error);
      throw new Error(`Failed to fetch cluster metadata for ${clusterName}`);
    }
  },

  /**
   * Produce a message to a Kafka topic
   */
  async produceMessage(
    clusterName: string,
    topicName: string,
    request: KafkaProduceRequest
  ): Promise<void> {
    try {
      await api.post(`/services/kafka/clusters/${clusterName}/topics/${topicName}`, request);
    } catch (error) {
      console.error(`Failed to produce message to topic ${topicName}:`, error);
      throw new Error(`Failed to produce message to topic ${topicName}`);
    }
  },
};
