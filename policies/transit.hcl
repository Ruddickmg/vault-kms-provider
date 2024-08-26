path "/transit/decrypt/*" {
  capabilities = ["update", "create"]
}
path "/transit/encrypt/*" {
  capabilities = ["update", "create"]
}
path "/transit/keys/*" {
  capabilities = ["read"]
}