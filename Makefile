helm:
	helm repo add bitnami https://charts.bitnami.com/bitnami
	helm repo update

clear_helm:
	helm repo remove bitnami

start_deps:
	helm install bitnami bitnami/kafka -n kafka --create-namespace -f helm/kafka/values-local.yaml

stop_deps:
	helm uninstall --ignore-not-found bitnami -n kafka
	kubectl delete --ignore-not-found pvc data-bitnami-kafka-controller-0 -n kafka