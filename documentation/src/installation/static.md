---
title: Static
---

## Static pod installation

> [!NOTE]
>
> - Vault KMS provider needs to be installed on the control plane node
>
> - Adding a static pod manifest for Kubernetes may be different for certain Kubernetes distros. Consult your distro's documentation if necessary.

Create a static pod configuration for the vault KMS provider, example below.
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: vault-kms-provider
  labels:
    app: vault-kms-provider
spec:
  volumes:
    - name: vault-kms-provider-socket
      hostPath:
        path: /mnt
        type: Directory
  containers:
    - name: vault-kms-provider
      image: "ruddickmg/vault-kms-provider:latest"
      env:
        - name: SOCKET_PATH
          value: "/sockets/vault-kms-provider.sock"
      volumeMounts:
        - name: vault-kms-provider-socket-volume
          mountPath: /sockets
```

Move the configuration file to `/etc/kubernetes/manifest`
```shell
mv /path/to/static-vault-kms-provider-config.yaml /etc/kubernetes/manifest
```

Restart kubernetes
```shell
systemctl restart kubelet
```

