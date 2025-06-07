from confluent_kafka.admin import AdminClient, NewTopic
import yaml
import os
from typing import List

def load_topic_config(file_path: str) -> List[dict]:
    try:
        with open(file_path, 'r') as file:
            config = yaml.safe_load(file)
            return config.get('topics', [])
    except Exception as e:
        print(f"Failed to load YAML file: {e}")
        return []

def create_kafka_topic():
    conf = {
        'bootstrap.servers': os.getenv('KAFKA_BOOTSTRAP_SERVERS', 'localhost:9092')
    }

    admin_client = AdminClient(conf)

    topic_configs = load_topic_config('topics_config.yaml')
    if not topic_configs:
        print("No topics found in YAML file")
        return

    new_topics = []
    for topic_config in topic_configs:
        for topic_name, config in topic_config.items():
            try:
                topic = NewTopic(
                    topic=topic_name,
                    num_partitions=config.get('partition', 1),
                    replication_factor=config.get('replication_factor', 1)
                )
                new_topics.append(topic)
            except Exception as e:
                print(f"Error preparing topic {topic_name}: {e}")

    if new_topics:
        try:
            futures = admin_client.create_topics(new_topics)
            for topic, future in futures.items():
                try:
                    future.result()
                    print(f"Successfully created topic: {topic}")
                except Exception as e:
                    print(f"Failed to create topic {topic}: {e}")
        except Exception as e:
            print(f"Failed to create topics: {e}")

    import time
    time.sleep(2)

    try:
        existing_topics = admin_client.list_topics(timeout=10).topics
        for topic in new_topics:
            if topic.topic in existing_topics:
                print(f"Topic '{topic.topic}' is listed in Kafka")
            else:
                print(f"Topic '{topic.topic}' not found")
    except Exception as e:
        print(f"Failed to list topics: {e}")

if __name__ == "__main__":
    create_kafka_topic()