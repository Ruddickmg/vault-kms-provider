# Vault KMS Provider

A plugin for kubernetes encryption that allows the use of vault as a KMS provider

### Usage:

See usage [documentation here](https://vault-kms-provider.io/)

### RoadMap:
- [x] Create grpc server from the [kubernetes proto file](https://kubernetes.io/docs/tasks/administer-cluster/kms-provider/#developing-a-kms-plugin-gRPC-server-kms-v2)
    - [x] Generate rust code from k8s proto
    - [x] Provide implementations for generated traits
    - [x] Get server in a running state
- [x] Create vault client for performing KMS actions
    - [x] Create all methods required for KMS service
    - [x] Test encryption with vault
    - [x] Test decryption with vault
    - [x] Test status checks (key info retrieval) with vault
- [x] Set up socket communication
    - [x] Connect to kubernetes kms provider via socket
    - [x] Secure Socket connection
    - [x] Test socket communication
- [x] Create docker container for plugin
- [x] Set up ci for deployment
- [ ] Set up Authentication
    - [ ] ServiceAccount
        - [x] Local
        - [ ] Client
        - [ ] Jwt
    - [ ] Tls certs
    - [x] Token
- [ ] Cache vault token
- [x] Create a docs page on github pages
- [ ] Allow Tls for http communication
- [x] Document manual integration steps
- [x] Create Helm Chart for easy deployment
- [x] Set up helm char repository via GitHub pages
- [x] Set up logging

## Kubernetes authentication

Official documentation on kubernetes authentication can be found
- [here in the vault docs](https://developer.hashicorp.com/vault/docs/auth/kubernetes)
- and [here in the Kubernetes docs](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#service-account-tokens).

### Service accounts

#### Local:

You can enable the use of local kubernetes authentication in vault using the following commands. [docs](https://developer.hashicorp.com/vault/docs/auth/kubernetes#use-local-service-account-token-as-the-reviewer-jwt)
```shell
vault auth enable kubernetes
vault write auth/kubernetes/config kubernetes_host=https://$KUBERNETES_SERVICE_HOST:$KUBERNETES_SERVICE_PORT
```

Then you will need to enable the service account, this is done via the values.yaml file for helm configuration. You can pass a custom values.yaml file with the service account property set to true, or use the `--set` flag
```shell
helm install vault-kms-provider --set "serviceAccount.enabled=true"
```

## Testing

### Unit
Unit tests should be colocated with the code they test, you can run unit tests using the following command
```shell
cargo test --bins --lib
```

### Integration
Integration tests will be located in the `tests` directory at the root of the project, they require some set up locally.

First you will need to run the vault service using docker compose by running the following command in the root directory:
```shell
docker compose up vault -d
```

Second you will need to enable transit in vault
```shell
docker compose exec vault vault secrets enable transit
```

After the transit has been enabled you can start the kms provider
```shell
docker compose up vault-kms-provider -d
```

Then finally you can run the integration tests with the following command
```shell
cargo test --test *
```

### End to end
End to end tests are implemented using helm's testing library, you can find the tests themselves in the `helm/templates/tests` directory. There are also some files used for testing located in the `helm/test_files` directory.

In order to run the tests you will need to deploy a vault server and the helm chart for this repository, you can do this using their respective helm charts

#### KMS provider chart installation
```shell
helm install vault-kms-provider ./helm -n vault \
  --set "image.tag=$IMAGE_TAG" \
  --set "role.rules[0].apiGroups={}" \
  --set "role.rules[0].resources={secrets}" \
  --set "role.rules[0].verbs={get,create}"
```
The parameters we set here are giving the tests the ability to create and modify secrets (so that we can check that they are encrypted/decrypted as expected). The image tag is self-explanatory, but keep in mind that if you are testing new changes, you will need to push them to a repository and specify the image you wish to pull and test in the `IMAGE_TAG` declaration.

#### Etcd
The tests expect there to be an etcd backend for them to access in order to confirm the stored data (secrets) is encrypted. There are too many potential k8s tools to cover how to make this work, but you may look to this repositories circleci config for examples on how to set one up if your brand of k8s does not use etcd. Either way, you will need to be sure the tests have access to an etcd backend to retrieve the data stored by k8s.

#### Running helm test

Once the vault KMS provider has been set up, you can run the tests with the following command:
```shell
helm test vault-kms-provider
```
