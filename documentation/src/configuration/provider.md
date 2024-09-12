---
title: Provider
---

## Provider

### Helm values

> [!NOTE]
> You can reference the helm [values.yaml](https://github.com/Ruddickmg/vault-kms-provider/blob/main/helm/values.yaml) for a full list of configurations

When deploying via helm, it is important to ensure that the `vault.address` is set correctly.

```shell
helm install vault-kms-provider --set "vault.address=https://vault.default.cluster.svc.local:8200"
```

Depending on the type of authentication you require you may want to disable the service account.

```shell
helm install vault-kms-provider --set "serviceAccount.create=false"
```

### Environment variables

Below are all the environment variables and their defaults for configuration of the KMS provider

```hcl
# Url of the vault service
VAULT_ADDRESS = "https://vault.vault.cluster.svc.local:8200"

# Path to the socket used for communication with the Kubernetes API server
SOCKET_PATH = "./sockets/vault-kms-provider.sock"

# The level of permissions granted to the socket, choices are:
#   - any: equivalent to 666
#   - user: equivalent to 600
#   - group: equivalent to 660
SOCKET_PERMISSIONS = "any"

# The string identifier used to store the encryption keys in the vault transit gateway
VAULT_TRANSIT_KEY = "vault-kms-provider"

# Used for authenticating with vault, only use if token authentication is desired.
VAULT_TOKEN = ""

# The endpoint that the health checks will listen on
HEALTH_ENDPOINT = "0.0.0.0:8080"
```