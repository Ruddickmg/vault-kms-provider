---
title: Kubernetes
---

### Kubernetes

> [!NOTE]
> This documentation is based on the  [official Kubernetes documentation](https://kubernetes.io/docs/tasks/administer-cluster/encrypt-data/#use-the-new-encryption-configuration-file).

You can generate a configuration with the default values to connect to the kms provider with the following command
```shell
helm template -s templates/configurations/encryption-configuration.yaml vault-kms-provider --set "encryption.output=true" > /etc/kubernetes/enc/enc.yaml
```

Or if you prefer to create one yourself, it looks like this
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

Then update the kubectl api server configuration to use this configuration.

`/etc/kubernetes/manifests/kube-apiserver.yaml`

```yaml
apiVersion: v1
kind: Pod
metadata:
  annotations:
    kubeadm.kubernetes.io/kube-apiserver.advertise-address.endpoint: 10.20.30.40:443
  creationTimestamp: null
  labels:
    app.kubernetes.io/component: kube-apiserver
    tier: control-plane
  name: kube-apiserver
  namespace: kube-system
spec:
  containers:
    - command:
        - kube-apiserver
        - --config-automatic-reload=true
        - --encryption-provider-config=/etc/kubernetes/enc/enc.yaml
      volumeMounts:
        - name: enc                          
          mountPath: /etc/kubernetes/enc
          readOnly: true
  volumes:
    - name: enc
      hostPath:
        path: /etc/kubernetes/enc
        type: DirectoryOrCreate
```

> [!IMPORTANT]
> The parameters from the above configuration should be added to the existing configuration (not override it).

Setting up kubernetes to use a KMS provider is different for each Kubernetes distro. See your distros docs for specifics if this documentation does not apply.
