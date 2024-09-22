---
title: TLS
---

## Set up TLS communication

> [!NOTE]
> Documentation on TLS configuration for vault can be [found on their website](https://developer.hashicorp.com/vault/docs/configuration/listener/tcp)

### Add Vault's CA file(s) to the KMS provider

To allow TLS encrypted communication with vault, Vault's CA (certificate authority) file(s) need to be installed in the KMS provider. This can be set in the values.yaml for the helm chart.

```yaml
vault:
  ca:
    # full path to a specific CA file
    file: "/path/to/ca.crt"
    # full path to a directory containing CA file(s)
    directory: "/path/to/ca/directory"
```

If you have a single CA file for vault, you can set a specific path to it.
```shell
helm install vault-kms-provider --set "vault.ca.file=/path/to/ca.crt"
```

If you have more than one file, or would rather just point to a directory, you can also specify a directory path. All certificates in the directory path will be installed in the KMS provider.
```shell
helm install vault-kms-provider --set "vault.ca.directory=/path/to/directory"
```

### Mount Vault's CA files into the KMS provider

In order for Vault's CA files to be installed, they must be present in the container, we can mount the CA file(s) by defining volumes and volumeMounts in our values.yaml.

The following are examples of how to mount Vault CA file(s) into the KMS provider.

#### Secrets

```yaml
volumes:
  # name of the volume, should match the volumeMount name
  - name: vault-ca-certificate
    secret:
      # The name of the secret that contains the Vault CA certificate(s)
      secretName: vault-ca-certs

volumeMounts:
  # Match with volume name
  - name: vault-ca-certificate
    # path where the files will be in the KMS provider
    mountPath: /etc/ssl/certs
```

#### Volumes

```yaml
volumes:
  # name of the volume, should match the volumeMount name
  - name: vault-ca-certificates
    hostPath:
      # Path to the directory where the Vault CA certificate(s) are located on the host machine
      path: /path/to/host/certificates
      type: Directory

volumeMounts:
  # Match with volume name
  - name: vault-ca-certificate
    # path where the files will be in the KMS provider
    mountPath: /etc/ssl/certs
```

> [!NOTE]
> A full list of [mount options and documentation](https://kubernetes.io/docs/concepts/storage/volumes/) can be found on the Kubernetes website. Chose what works best for your use case.