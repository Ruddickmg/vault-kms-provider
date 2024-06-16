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

## Kubernetes authentication

Official documentation on kubernetes authentication can be found 
- here in the vault docs 
- and here in the Kubernetes docs.

In order to access vault the kms provider will need to authenticate with it. In order for authentication to work via kubernetes, you will need a few things:
1. The CA used by kubernetes, this can be retrieved using the following command, which will output a file `ca.crt` containing the kubernetes CA certificate
```shell
kubectl config view --raw --minify --flatten -o jsonpath='{.clusters[].cluster.certificate-authority-data}' | base64 --decode > ca.crt
```
2. 