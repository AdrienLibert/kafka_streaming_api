helm:
	helm repo add bitnami https://charts.bitnami.com/bitnami
	helm repo update

clear_helm:
	helm repo remove bitnami

build:
	docker build -t local/kafka -f src/kafka-init/Dockerfile src/kafka-init/
	docker build -t local/producer -f src/kafka-producer-api/Dockerfile src/kafka-producer-api/
	docker build -t local/consumer -f src/kafka-consumer/Dockerfile src/kafka-consumer/


start:
	helm install bitnami bitnami/kafka -n streaming --create-namespace -f helm/kafka/values-local.yaml
	kubectl apply -f k8s/job_kafka.yaml
	kubectl apply -f k8s/producer.yaml
	kubectl apply -f k8s/consumer.yaml

stop:
	helm uninstall --ignore-not-found bitnami -n streaming
	kubectl delete --ignore-not-found pvc data-bitnami-kafka-controller-0 -n streaming
	kubectl delete -f k8s/job_kafka.yaml -n streaming --ignore-not-found
	kubectl delete -f k8s/producer.yaml -n streaming --ignore-not-found
	kubectl delete -f k8s/consumer.yaml -n streaming --ignore-not-found