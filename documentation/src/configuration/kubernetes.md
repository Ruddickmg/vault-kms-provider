---
title: Kubernetes
tags:
  - quick-start
---

## Configure Kubernetes

> [!NOTE]
> Kubernetes documentation on setting up encryption can be found [here](https://kubernetes.io/docs/tasks/administer-cluster/encrypt-data/#use-the-new-encryption-configuration-file)

Create an encryption configuration for the Kubernetes api server

`./encryption-configuration.yaml`
```yaml
apiVersion: apiserver.config.k8s.io/v1
kind: EncryptionConfiguration
resources:
  - resources:
      - secrets
    providers:
      - kms:
          apiVersion: v2
          name: vault-kms-provider
          endpoint: unix:///mnt/vault-kms-provider.sock
          timeout: 3s
      - identity: {}
```

Point the api server to your encryption configuration

`/etc/kubernetes/manifests/kube-apiserver.yaml`
```yaml
# add these commands to your Kubernetes api server configuration
spec:
  containers:
    - command:
        - kube-apiserver
        # Point to your encryption file
        - --encryption-provider-config="/path/to/your/encryption-configuration.yaml"
```

This is done in differently in some flavors of kubernetes, if yours is different, consult the documentation of your Kubernetes distro for instructions on how to point Kubernetes to your configuration file.
