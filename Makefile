helm:
	helm repo add bitnami https://charts.bitnami.com/bitnami
	helm repo update

clear_helm:
	helm repo remove bitnami

build:
	docker build -t local/kafka -f src/kafka/Dockerfile src/kafka/

start_deps:
	helm install bitnami bitnami/kafka -n streaming --create-namespace -f helm/kafka/values-local.yaml
	kubectl apply -f k8s/job_kafka.yaml

stop_deps:
	helm uninstall --ignore-not-found bitnami -n streaming
	kubectl delete --ignore-not-found pvc data-bitnami-kafka-controller-0 -n streaming