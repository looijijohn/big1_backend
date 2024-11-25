#!/bin/bash

# Create Kubernetes configuration files

cat <<EOF > user-service-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: user-service
  labels:
    app: user-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: user-service
  template:
    metadata:
      labels:
        app: user-service
    spec:
      containers:
      - name: user-service
        image: your-docker-registry/user-service:latest
        ports:
        - containerPort: 8080
        env:
        - name: MONGO_URI
          valueFrom:
            secretKeyRef:
              name: mongo-secret
              key: mongo_uri
        resources:
          requests:
            memory: "128Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "1"
EOF

cat <<EOF > user-service-service.yaml
apiVersion: v1
kind: Service
metadata:
  name: user-service
  labels:
    app: user-service
spec:
  type: ClusterIP
  ports:
  - port: 80
    targetPort: 8080
  selector:
    app: user-service
EOF

cat <<EOF > user-service-configmap.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: user-service-config
data:
  MONGO_URI: mongodb://your-mongo-service:27017
EOF

cat <<EOF > mongo-secret.yaml
apiVersion: v1
kind: Secret
metadata:
  name: mongo-secret
type: Opaque
data:
  mongo_uri: bW9uZ29kYjovL3lvdXItbW9uZ28tc2VydmljZToyNzAxNw==
EOF

echo "Kubernetes configuration files created successfully."