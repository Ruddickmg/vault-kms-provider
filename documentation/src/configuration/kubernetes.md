---
title: Kubernetes
---

### Kubernetes

> [!NOTE]
> Official docs for encryption configuration can be found [here](https://kubernetes.io/docs/tasks/administer-cluster/encrypt-data/)

Create an encryption configuration for Kubernetes
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

Then update the kubectl api server configuration to use this file. This may be different depending on your Kubernetes distro. Below are examples from different distros but the list is not exhaustive, if your distro is not included then consult the documentation.
<details>
  <summary>Kubernetes</summary>

For Kubernetes, update the `/etc/kubernetes/manifests/kube-apiserver.yaml` file by adding the following to the `spec` section

```yaml
spec:
  containers:
    - command:
        - kube-apiserver
        # tell Kubernetes where to find the encryption configuration
        - --encryption-provider-config=/etc/kubernetes/enc/enc.yaml
      # add a volume that points to the configuration file
      volumeMounts:
        - name: enc                          
          mountPath: /etc/kubernetes/enc
          readOnly: true
  # add a volume that points to the configuration file
  volumes:
    - name: enc
      hostPath:
        path: /etc/kubernetes/enc
        type: DirectoryOrCreate
```

 </details>
<details>
  <summary>K3s</summary>

   In K3s you can specify the location of the encryption configuration file via command line arguments

  ```bash
    curl -sfL https://get.k3s.io | sh -s - --kube-apiserver-arg=encryption-provider-config=/etc/kubernetes/enc/enc.yaml
  ```
</details>