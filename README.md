# Vault KMS Provider

A plugin for kubernetes encryption that allows the use of vault as a KMS provider

### Usage:

See usage [documentation here](https://vault-kms-provider.io/)

### RoadMap:
- [ ] Set up Authentication
  - [x] Make optional file path or value for each secret
  - [x] Kubernetes
  - [x] Token
  - [x] User & Password
  - [x] App Role
  - [ ] JWT
  - [ ] Tls certs
  - [ ] OIDC

## Kubernetes authentication

> Official documentation on kubernetes authentication can be found
> - [here in the vault docs](https://developer.hashicorp.com/vault/docs/auth/kubernetes)
> - [here in the Kubernetes docs](https://kubernetes.io/docs/reference/access-authn-authz/authentication/#service-account-tokens).

### Service accounts

You can enable the use of local kubernetes authentication in vault using the following commands. [docs](https://developer.hashicorp.com/vault/docs/auth/kubernetes#use-local-service-account-token-as-the-reviewer-jwt)
```shell
vault auth enable kubernetes
vault write auth/kubernetes/config kubernetes_host=https://$KUBERNETES_SERVICE_HOST:$KUBERNETES_SERVICE_PORT
```

## Testing

### Unit
Unit tests should be colocated with the code they test, you can run unit tests using the following command
```shell
cargo test --bins --lib
```

### Integration
Integration tests will be located in the `tests` directory at the root of the project, they require some set up locally.

#### Running Vault & The KMS provider
First, you will need to run the vault service using docker compose by running the following command in the root directory:
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

#### Setting up Authentication

You'll also need to enable the authentication methods which are tested via integration tests

Enable userpass authentication
```shell
docker compose exec vault vault auth enable userpass
```

Create user & password for userpass authentication
```shell
docker compose exec vault vault write auth/userpass/users/vault-kms-provider \
    password=password \
    policies=default
```

Enable app role authentication
```shell
docker compose exec vault vault auth enable approle
```

Generate a role id
```shell
docker compose exec vault vault write auth/approle/role/vault-kms-provider \
    token_type=batch \
    secret_id_ttl=10m \
    token_ttl=20m \
    token_max_ttl=30m \
    secret_id_num_uses=40
```

Output the role id to a file
```shell
docker compose exec vault vault read auth/approle/role/vault-kms-provider/role-id -format="json" | jq -r .data.role_id > ./test_files/role_id
```

Output the secret id to a file
```shell
docker compose exec vault vault write -f auth/approle/role/vault-kms-provider/secret-id -format="json" | jq -r .data.secret_id > ./test_files/secret_id
```

#### Tests

Then finally you can run the integration tests with the following command
```shell
cargo test --test *
```

> [!NOTE]
> you can output logs for debugging vault into console by running the following command
> ```shell
> docker compose exec vault vault audit enable file file_path=stdout
> ```

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

The tests will need to be able to make requests to etcd. In order to communicate over tls, you will need to point to tls certificates that can verify communication with Etcd. The tests run by helm will mount a volume at `/tmp/certs` where it will look for three files:
`ca.crt`, `tls.crt`, and `tls.key`. With these certificates in place the tests should run fine over tls.

#### Running helm test

Once the vault KMS provider has been set up, you can run the tests with the following command:
```shell
helm test vault-kms-provider
```

## Tls

Tls is enabled by setting environment variables that point to the relevant certificate files required for tls.

###  environment variables

```shell
# Defines the path to a ca certificate file
VAULT_CA_CERT="/path/to/cert/file.crt"

# Defines the path to a directory containing ca certificate file
VAULT_CA_PATH="/path/to/cert/directory"
```