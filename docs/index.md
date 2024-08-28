# Vault KMS Provider

## Configuration

Listed below are a few things that must be configured in order for the KMS provider to be installed

### Vault

#### Encryption

You will need to enable the transit gateway in Vault in order to use it to encrypt and decrypt data for Kubernetes. You can do so with the following command:
```shell
kubectl exec vault-0 -- vault secrets enable transit
```

#### Authentication

Vault needs to be configured to allow the KMS provider to connect to it, the default method of authentication is kubernetes authentication via service accounts.

In order 

```shell

```

### Kubernetes

### KMS Provider

You can reference the helm [values.yaml](https://github.com/Ruddickmg/vault-kms-provider/blob/main/helm/values.yaml) for a full list of configurations



## Installation

Add the helm repository
```shell
helm add https://vault-kms-provider.io 
```

Install the helm chart
```shell
helm install vault-kms-provider
```


