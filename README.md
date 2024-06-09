# Vault KMS Provider

A plugin for kubernetes encryption that allows the use of vault as a KMS provider

### RoadMap:
  - [x] Create grpc server from the [kubernetes proto file](https://kubernetes.io/docs/tasks/administer-cluster/kms-provider/#developing-a-kms-plugin-gRPC-server-kms-v2)
    - [x] Generate rust code from k8s proto
    - [x] Provide implementations for generated traits
    - [x] Get server in a running state
  - [ ] Create vault client for performing KMS actions
    - [x] Create all methods required for KMS service
    - [ ] Test encryption with vault
    - [ ] Test decryption with vault
    - [ ] Test status checks (key info retrieval) with vault
    - [ ] Set up Authentication
      - [ ] ServiceAccount
      - [ ] Jwt
      - [ ] Test authentication methods
  - [ ] Ensure Tls is used for all communication
  - [ ] Set up socket communication
    - [ ] Connect to kubernetes kms provider via socket
    - [ ] Secure Socket connection
    - [ ] Test socket communication
  - [ ] Create docker container for plugin
  - [ ] Set up ci for deployment
  - [ ] Create Helm Chart for easy deployment
  - [ ] Document manual integration steps

