---
title: Authentication
---

## Authentication

Currently, the following authentication methods are supported

- [Token](https://developer.hashicorp.com/vault/api-docs/auth/token)
- [Kubernetes](https://developer.hashicorp.com/vault/docs/auth/kubernetes)
- [UserPass](https://developer.hashicorp.com/vault/docs/auth/userpass)
- [AppRole](https://developer.hashicorp.com/vault/docs/auth/approle)
- [JWT/OIDC](https://developer.hashicorp.com/vault/docs/auth/jwt)
- [Certificate](https://developer.hashicorp.com/vault/docs/auth/cert)

Configuration of auth methods is done using the environment variables listed below.

```hcl
# Path defined for the authentication route, ex: auth/custom-auth-path/...
#  if not set, will default to the associated auth method, ex: auth/userpass/.. or auth/kubernetes/..
VAULT_AUTH_MOUNT = "custom-auth-path"

# Vault token for vault access
VAULT_TOKEN = "SiQOECxwSDCeQt1r0n5kqQCr"
# path to file containing vault token
VAULT_TOKEN_PATH = "/path/to/vault/token"

# user and password for userpass authentication
VAULT_USER = "vault-kms-provider"
VAULT_PASSWORD = "some-password"
# path to file containing vault password
VAULT_PASSWORD_PATH = "/path/to/vault/password"

# path to mounted JWT for kubernetes auth
VAULT_KUBERNETES_JWT_PATH = "/path/to/vault.jwt"
# jwt for kubernetes auth
VAULT_KUBERNETES_JWT = "jwt"
# role for kubernetes auth 
VAULT_KUBERNETES_ROLE = "vault-kms-provider"

# role_id and secret_id for approle authentication
VAULT_ROLE_ID = "role"
VAULT_SECRET_ID = "secret"
# path to file containing secret id
VAULT_SECRET_ID_PATH = "/path/to/secret/id"

# jwt for jwt auth
VAULT_JWT = "jwt"
# path to mounted jwt for jwt auth
VAULT_JWT_PATH = "/path/to/jwt"
# role for jwt, optional
VAULT_JWT_ROLE = "vault-kms-provider"

# name of the trusted certificate created in vault for authentication
VAULT_CERTIFICATE_NAME = "vault-kms-provider"
# path to client cert and key for certificate authentication
VAULT_CLIENT_CERT = "/path/to/client/public.crt"
VAULT_CLIENT_KEY = "/path/to/client/private.key"
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