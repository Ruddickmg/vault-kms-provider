---
title: Authentication
---

## Authentication

Currently, the following authentication methods are supported

- [Token](https://developer.hashicorp.com/vault/api-docs/auth/token)
- [Kubernetes](https://developer.hashicorp.com/vault/docs/auth/kubernetes)
- [Userpass](https://developer.hashicorp.com/vault/docs/auth/userpass)

Configuration of auth methods is done using the environment variables listed below.

```hcl
# Path defined for the authentication route, ex: auth/custom-auth-path/...
#  if not set, will default to the associated auth method, ex: auth/userpass/.. or auth/kubernetes/..
VAULT_AUTH_PATH = "custom-auth-path"

# Vault token for vault access
VAULT_TOKEN = "SiQOECxwSDCeQt1r0n5kqQCr"

# user and password for userpass authentication
VAULT_USER = "vault-kms-provider"
VAULT_PASSWORD = "some-password"

# path to mounted JWT for kubernetes auth
VAULT_JWT_PATH = "/path/to/vault.jwt"
```

Environment variables can be configured using the `env` property in the values.yaml, ex:

```yaml
env:
  - name: VAULT_TOKEN
    valueFrom:
      secretKeyRef:
        name: vault-token-secret
        key: token
```

The `envFrom` property is also configurable in the values.yaml file, allowing configMaps, etc to be added. ex:

```yaml
envFrom:
  - configMapRef:
    name: my-custom-config-map
```