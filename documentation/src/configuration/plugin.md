---
title: Plugin
---

## Plugin

### Helm values

> [!NOTE]
> You can reference the helm [values.yaml](https://github.com/Ruddickmg/vault-kms-provider/blob/main/helm/values.yaml) for a full list of configurations

When deploying via helm, it is important to ensure that the `vault.address` is set correctly.

```shell
helm install vault-kms-provider --set "vault.address=https://vault.default.svc.cluster.local:8200"
```

Depending on the type of authentication you require you may want to disable the service account.

```shell
helm install vault-kms-provider --set "serviceAccount.create=false"
```

### Environment variables

Below are some general environment variables and their defaults for configuration of the KMS provider

```hcl
# Url of the vault service
VAULT_ADDRESS = "https://vault.vault.svc.cluster.local:8200"

# The endpoint that the health checks will listen on
HEALTH_ENDPOINT = "0.0.0.0:8080"

# Path to the socket used for communication with the Kubernetes API server. Can be either abstract (@path/to/abstract.sock) or file path.
# Abstract socket paths must be prefixed with the "@" symbol
SOCKET_PATH = "./sockets/vault-kms-provider.sock"

# The level of permissions granted to the socket (does not apply to abstract sockets)
SOCKET_PERMISSIONS = "666"

# The string identifier used to store the encryption keys in the vault transit gateway
VAULT_TRANSIT_KEY = "vault-kms-provider"

# path defined for the transit gateway, ex: auth/transit/... or auth/transit-path/...
VAULT_TRANSIT_MOUNT = "transit"
```