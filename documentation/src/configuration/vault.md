---
title: Vault
tags:
  - quick-start
---

## Set up Vault

> [!NOTE]
> The Vault KMS Provider will use any transit key found at the default or user specified transit path, if no key is found the provider will initialize one with Vault transits default key type ([aes256-gcm96](https://developer.hashicorp.com/vault/api-docs/secret/transit#aes256-gcm96)).
> 
> See [Vault documentation](https://developer.hashicorp.com/vault/api-docs/secret/transit#create-key) creating keys.

### Encryption

Enable the transit gateway in Vault for encryption/decryption of data for Kubernetes.
```shell
vault secrets enable transit
```

Create a policy granting the permissions to the KMS provider to encrypt/decrypt data.

`./transit.hcl`
```hcl
path "/transit/decrypt/vault-kms-provider" {
  capabilities = ["update", "create"]
}
path "/transit/encrypt/vault-kms-provider" {
  capabilities = ["update", "create"]
}
path "/transit/keys/vault-kms-provider" {
  capabilities = ["read"]
}
```

Add the policy to vault
```shell
vault policy write vault-kms-provider transit.hcl
```

### Authentication

Enable authentication via kubernetes
```shell
vault auth enable kubernetes
```

Set the host URL to the kubernetes API
```shell
vault write auth/kubernetes/config kubernetes_host="https://kubernetes.default.svc/"
```

Create a role for the KMS provider's service account so that it can authenticate with vault.
```shell
vault write auth/kubernetes/role/vault-kms-provider \
    bound_service_account_names=vault-kms-provider \
    bound_service_account_namespaces=default \
    audience=vault \
    token_policies=vault-kms-provider \
    ttl=1h
```

With vault configured you should be able to deploy the vault-kms-provider to kubernetes without error.
