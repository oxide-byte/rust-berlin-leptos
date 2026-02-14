# KeyCloak

## References

https://datatracker.ietf.org/doc/html/rfc7519

https://www.jwt.io/

https://www.keycloak.org/ui-customization/themes

## JWT Sample

```json
{
  "exp": 1771066733,
  "iat": 1771066433,
  "jti": "onrtro:0dffc9b4-6671-20e7-b436-78335abc7f42",
  "iss": "http://127.0.0.1:8888/realms/hackandlearn",
  "sub": "4090673d-7a8c-4dbe-8f40-d6273fc4ae4b",
  "typ": "Bearer",
  "azp": "hackandlearn-client",
  "sid": "dw0OpuinX3Nn8hThspiqFZGd",
  "acr": "1",
  "allowed-origins": [
    "http://127.0.0.1",
    "http://localhost"
  ],
  "resource_access": {
    "hackandlearn-client": {
      "roles": [
        "ROLE_HNL_USER"
      ]
    }
  },
  "scope": "email profile",
  "email_verified": true,
  "company_id": "e41cb811-9eec-46bd-af41-dd43d24b378d",
  "company_name": "rustician",
  "name": "user user",
  "preferred_username": "user@demo.com",
  "given_name": "user",
  "family_name": "user",
  "email": "user@demo.com"
}
```

## Export / Import Configuration

### Export

Connection in the running docker instance

Export Config + User [Link](https://www.keycloak.org/server/importExport)

```Shell
cd /opt/keycloak/bin
./kc.sh export --file /tmp/export.json --realm hackandlearn
```

### Import

Copy `export.json` to `dev_infra/keycloak/realm.json`

Delete all Docker images, volumes and restart.