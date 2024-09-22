listener "tcp" {
  address = "0.0.0.0:8400"
  tls_disable = false
  tls_cert_file = "/vault/userconfig/tls-server/tls.crt"
  tls_key_file = "/vault/userconfig/tls-server/tls.key"
}