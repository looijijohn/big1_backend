# KCEX - Cryptocurrency Exchange

KCEX is a microservices-based cryptocurrency exchange platform designed to be scalable, secure, and highly performant. It provides functionalities like user authentication, trading, order management, wallet services, notification services, and more.

## Table of Contents
- [Architecture](#architecture)
- [Microservices](#microservices)
- [Technology Stack](#technology-stack)
- [Setup](#setup)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Running the Project](#running-the-project)
- [Deployment](#deployment)
  - [Docker Compose](#docker-compose)
  - [Kubernetes](#kubernetes)
- [Monitoring and Logging](#monitoring-and-logging)
- [CI/CD Pipeline](#cicd-pipeline)
- [Contributing](#contributing)
- [License](#license)

## Architecture

The platform follows a microservices architecture, where each service is responsible for a specific functionality. The API Gateway (Traefik) handles routing to various microservices.

## Microservices

1. **API Gateway (Traefik)**
   - Central entry point, routes traffic to microservices.

2. **Auth Service**
   - Manages user authentication and authorization.
   - **Models:** Users, Tokens

3. **User Service**
   - Manages user profiles and information.
   - **Models:** User Profiles

4. **Trading Service**
   - Manages trading operations with an in-memory order book.
   - **Models:** Orders, Trades

5. **Order Service**
   - Manages user orders.
   - **Models:** Orders

6. **Wallet Service**
   - Manages user wallets and balances.
   - **Models:** Wallets, Transactions

7. **Notification Service**
   - Sends notifications to users.
   - **Models:** Notifications

8. **Coin and Pair Service**
   - Manages cryptocurrency coins and trading pairs.
   - **Models:** Coins, Pairs

## Technology Stack

- **Backend:** Go (Gin Framework)
- **Database:** MongoDB
- **Cache:** Redis
- **Containerization:** Docker
- **Orchestration:** Kubernetes
- **API Gateway:** Traefik
- **CI/CD:** GitHub Actions, Jenkins, or Argo
- **Monitoring:** Prometheus, Grafana
- **Logging:** ELK Stack (Elasticsearch, Logstash, Kibana)

## Setup

### Prerequisites
- Docker
- Docker Compose
- Kubernetes and kubectl
- Helm (optional, for managing Kubernetes applications)

### Installation
1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/kcex.git
    cd kcex
    ```

### Running the Project

#### Docker Compose
1. Start the services using Docker Compose:
    ```sh
    docker-compose up -d
    ```

#### Kubernetes
1. Apply the Kubernetes manifests:
    ```sh
    kubectl apply -f kubernetes/
    ```

## Deployment

### Docker Compose
1. Ensure Docker and Docker Compose are installed.
2. Start the services:
    ```sh
    docker-compose up -d
    ```

### Kubernetes
1. Ensure Kubernetes is set up and kubectl is configured.
2. Apply the manifests:
    ```sh
    kubectl apply -f kubernetes/
    ```

## Monitoring and Logging
- **Monitoring:** Prometheus and Grafana are used for monitoring the health and performance of the services.
- **Logging:** The ELK Stack is used for centralized logging and alerting.

## CI/CD Pipeline
- **CI/CD Tools:** GitHub Actions, Jenkins, or Argo can be used for automating the build, test, and deployment processes.
- **Pipeline Configuration:** Detailed configuration can be found in the `.github/workflows/` or `Jenkinsfile` or `ArgoCD` directory.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any changes.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.


```                          +------------------------+
                          |      API Gateway       |
                          |         (Traefik)      |
                          +-----------+------------+
                                      |
        +-------------------+---------+-------------+------------------+
        |                   |                       |                  |
+-------+--------+   +------+-------+    +----------+---------+  +-----+-----+
|    Auth        |   |   User       |    | Trading (In-Memory) |  |Notification |
|  Service       |   |   Service    |    |  Service            |  |  Service    |
|(Go, Gin)       |   |  (Go, Gin)   |    |  (Go, Gin)          |  |  (Go, Gin)  |
+-------+--------+   +------+-------+    +----+----+-----------+  +-----+-----+
|Model: Users    |   |Model: Profiles|   |Models: Orders, Trades| |Model: Notif.|
|      Tokens    |   |                |   +----+---+------------+  |            |
+----------------+   +----------------+        |   |               +------------+
                                                   |
        +----------------------+        +----------+-----------+   +------------+
        |  Coin & Pair Service |        | Order    | Wallet    |   |Analytics    |
        |  (Go, Gin)           |        | Service  | Service   |   |Service      |
        |  Model: Coins, Pairs |        | (Go, Gin)| (Go, Gin) |   |(Go, Gin)    |
        +----------+-----------+        | Models: Orders       |   |Model: Reports|
                   |                    | Models: Wallets, Txns|   +-------------+
                   |                    +----------+-----------+
                   +------------------------------------------------+
                   |        Microservices Management Layer          |
                   |                (Kubernetes)                    |
                   +-----------------+------------------------------+
                                      |
                    +-----------------|--------------------+
                    |                 |                    |
+-------------------+------+   +------+--------------+  +--+---------------+
|CI/CD Pipeline            |   |Monitoring          |  |Logging & Alerting|
|(GitHub Actions/Jenkins)  |   |(Prometheus/Grafana)|  |(ELK Stack)       |
+--------------------------+   +--------------------+  +------------------+
```