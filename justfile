default: set_up_environment set_up_permissions set_up_authentication enable_logs

set_up_permissions: configure_transit_access
set_up_environment: start_vault enable_transit start_kms_provider
set_up_authentication: set_up_userpass_auth set_up_approle_auth set_up_jwt_auth set_up_cert_auth

set_up_jwt_auth: enable_jwt_authentication configure_jwt_authentication set_up_jwt_role
set_up_approle_auth: enable_approle_authentication confiugre_approle_authentication
set_up_userpass_auth: enable_userpass_authentication configure_userpass_authentication
set_up_cert_auth: enable_cert_authentication configure_cert_authentication

start_vault:
  docker compose up vault -d
  sleep 1

start_kms_provider:
  docker compose up vault-kms-provider -d

enable_transit:
  docker compose exec vault vault secrets enable transit

enable_userpass_authentication:
  docker compose exec vault vault auth enable userpass

configure_userpass_authentication:
  docker compose exec vault vault write auth/userpass/users/vault-kms-provider \
      password=password \
      policies=vault-kms-provider

enable_approle_authentication:
  docker compose exec vault vault auth enable approle

confiugre_approle_authentication: create_approle_role store_approle_role_id store_approle_secret_id

create_approle_role:
  docker compose exec vault vault write auth/approle/role/vault-kms-provider \
      token_type=batch \
      secret_id_ttl=10m \
      token_ttl=20m \
      token_max_ttl=30m \
      secret_id_num_uses=40

store_approle_role_id:
  docker compose exec vault vault read auth/approle/role/vault-kms-provider/role-id -format="json" | jq -r .data.role_id > ./test_files/role_id

store_approle_secret_id:
  docker compose exec vault vault write -f auth/approle/role/vault-kms-provider/secret-id -format="json" | jq -r .data.secret_id > ./test_files/secret_id

enable_jwt_authentication:
  docker compose exec vault vault auth enable jwt

set_up_jwt_role:
  docker compose exec vault vault write auth/jwt/role/vault-kms-provider policies="vault-kms-provider" user_claim="sub" role_type="jwt" bound_audiences="vault"

configure_jwt_authentication:
  docker compose exec vault vault write auth/jwt/config jwt_supported_algs=RS256 jwt_validation_pubkeys="$(cat ./test_files/jwt/public_key.pem)"

enable_cert_authentication:
  docker compose exec vault vault auth enable cert

configure_cert_authentication:
  docker compose exec vault vault write auth/cert/certs/vault-kms-provider \
    display_name=vault-kms-provider \
    policies=vault-kms-provider \
    certificate="$(cat ./test_files/certs/tls.crt)" \
    ttl=3600

configure_transit_access:
    docker compose exec vault vault policy write vault-kms-provider /policies/transit.hcl

enable_logs:
   docker compose exec vault vault audit enable file file_path=stdout