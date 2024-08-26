path "/transit/decrypt/vault-kms-provider" {
  capabilities = ["update", "create"]
}
path "/transit/encrypt/vault-kms-provider" {
  capabilities = ["update", "create"]
}
path "/transit/keys/vault-kms-provider" {
  capabilities = ["read"]
}
