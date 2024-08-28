# Vault KMS Provider

## Installation

Add the helm repository
```shell
helm add https://vault-kms-provider.io 
```

Install the helm chart
```shell
helm install vault-kms-provider
```

## Configuration

Listed below are a few things that must be configured in order for the KMS provider to be installed

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

### Kubernetes

Setting up kubernetes to use a KMS provider is different for each Kubernetes distro but documentation for kubernetes can be [found here](https://kubernetes.io/docs/tasks/administer-cluster/encrypt-data/#use-the-new-encryption-configuration-file). The following configuration is based on this documentation. 

You can generate a configuration with the default values to connect to the kms provider with the following command
```shell
helm template -s templates/configurations/encryption-configuration.yaml vault-kms-provider --set "encryption.output=true" > /etc/kubernetes/enc/enc.yaml
```

Or if you prefer to create one yourself, it looks like this
```yaml

```

Then update the kubectl api server configuration to use this configuration.

`/etc/kubernetes/manifests/kube-apiserver.yaml`

### KMS Provider

You can reference the helm [values.yaml](https://github.com/Ruddickmg/vault-kms-provider/blob/main/helm/values.yaml) for a full list of configurations



