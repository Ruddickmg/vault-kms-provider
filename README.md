![GitHub License](https://img.shields.io/github/license/Ruddickmg/vault-kms-provider)
![CircleCI (branch)](https://img.shields.io/circleci/build/github/Ruddickmg/vault-kms-provider/main)
![Codecov (with branch)](https://img.shields.io/codecov/c/github/Ruddickmg/vault-kms-provider/main?logo=codecov)
![GitHub Release](https://img.shields.io/github/v/release/Ruddickmg/vault-kms-provider)

# Vault KMS Provider

A plugin for kubernetes encryption at rest that allows the use of vault as a KMS provider

### Usage:

See usage [documentation here](https://vault-kms-provider.io/)

## Contributing

### Versioning

All commits must use conventional commit messages to ensure versioning happens correctly. 

### Road map
- [ ] Encryption
  - [ ] Support convergent encryption
  - [ ] Support key derivation

## Testing

### Unit
Unit tests should be colocated with the code they test, you can run unit tests using the following command
```shell
cargo test --bins --lib
```

### Integration
Integration tests will be located in the `tests` directory at the root of the project, they require some set up locally.

Make sure just is installed on your system
```shell
cargo install just
```

Run the justfile to set up the environment
```shell
just
```

Then you can run the integration tests with the following command
```shell
cargo test --test *
```

### End to end
End to end tests are implemented using helm's testing library, you can find the tests themselves in the `helm/templates/tests` directory. There are also some files used for testing located in the `test_files` directory.

In order to run the tests you will need to deploy a vault server and the helm chart for this repository, you can do this using their respective helm charts

#### KMS provider chart installation
```shell
helm install vault-kms-provider ./helm -n vault \
  --set "image.tag=$IMAGE_TAG" \
  --set "role.rules[0].apiGroups={}" \
  --set "role.rules[0].resources={secrets}" \
  --set "role.rules[0].verbs={get,create}"
```
The parameters we set here are giving the tests the ability to create and modify secrets (so that we can check that they are encrypted/decrypted as expected). The image tag is self-explanatory, but keep in mind that if you are testing new changes, you will need to push them to a repository and specify the image you wish to pull and test in the `IMAGE_TAG` declaration.

#### Etcd
The tests expect there to be an etcd backend for them to access in order to confirm the stored data (secrets) is encrypted. There are too many potential k8s tools to cover how to make this work, but you may look to this repositories circleci config for examples on how to set one up if your brand of k8s does not use etcd. Either way, you will need to be sure the tests have access to an etcd backend to retrieve the data stored by k8s.

The tests will need to be able to make requests to etcd. In order to communicate over tls, you will need to point to tls certificates that can verify communication with Etcd. The tests run by helm will mount a volume at `/tmp/certs` where it will look for three files:
`ca.crt`, `tls.crt`, and `tls.key`. With these certificates in place the tests should run fine over tls.

#### Running helm test

Once the vault KMS provider has been set up, you can run the tests with the following command:
```shell
helm test vault-kms-provider
```

K3s can be debugged using the following command:
```shell
journalctl -xefu k3s.service
```

## Tls

Tls is enabled by setting environment variables that point to the relevant certificate files required for tls.

###  environment variables

```shell
# Defines the path to a ca certificate file
VAULT_CA_CERT="/path/to/cert/file.crt"

# Defines the path to a directory containing ca certificate file
VAULT_CA_PATH="/path/to/cert/directory"
```