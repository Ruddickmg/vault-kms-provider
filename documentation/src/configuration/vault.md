### Vault

#### Encryption

You will need to enable the transit gateway in Vault in order to use it to encrypt and decrypt data for Kubernetes. You can do so with the following command:
```shell
vault secrets enable transit
```

Once you have enabled the transit gateway you will need to create a policy granting the permissions required for the KMS provider to encrypt and decrypt data. This can be done by creating a policy file

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

Create the policy in vault with the following command
```shell
vault policy write vault-kms-provider transit.hcl
```

#### Authentication

Vault needs to be configured to allow the KMS provider to connect to it, the default method of authentication is kubernetes authentication via service accounts.

In order to use this authentication method you will need to enable it with the following command.
```shell
vault auth enable kubernetes
```

You will then need to give vault the url of the kubernetes api so that it can use it to authenticate with.
```shell
vault write auth/kubernetes/config kubernetes_host="https://kubernetes.default.svc/"
```

Finally, you will need to create a role for the KMS provider's service account so that it can authenticate with vault.
```shell
vault write auth/kubernetes/role/vault-kms-provider \
    bound_service_account_names=vault-kms-provider \
    bound_service_account_namespaces=default \
    audience=vault \
    token_policies=vault-kms-provider \
    ttl=1h
```

With vault configured you should be able to deploy the vault-kms-provider to kubernetes without error.