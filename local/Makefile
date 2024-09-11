deploy-db:
	-kubectl delete -f ./pgadmin-deployment.yaml
	-kubectl delete -f ./pgadmin-svc.yaml
	-kubectl delete -f ./db-postgres.yaml
	-kubectl delete -f ./db-svc.yaml
	-kubectl delete -f ./configmap.yaml
	kubectl apply -f ./configmap.yaml
	kubectl apply -f ./db-postgres.yaml
	kubectl apply -f ./db-svc.yaml
	kubectl apply -f ./pgadmin-deployment.yaml
	kubectl apply -f ./pgadmin-svc.yaml

deploy-api:
	-kubectl delete -f ./app-svc.yaml
	-kubectl delete -f ./app-deployment.yaml
	kubectl apply -f ./app-deployment.yaml
	kubectl apply -f ./app-svc.yaml

build-image:
	docker build -t mblopes/tech-challenge-pedido-produto:latest ../
	docker push mblopes/tech-challenge-pedido-produto:latest
	docker rmi mblopes/tech-challenge-pedido-produto:latest

full-local-deploy: build-image deploy-api
