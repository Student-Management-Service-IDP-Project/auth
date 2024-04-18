## Authentication Service

###  __API__

#### `LOGIN USER REQUEST`
---
`> Request-type:` _POST_

`> Url`: http://127.0.0.1:3000/user/login

`> Content-type:` __x-www-form-urlencoded__

`> Body:`
```
    username: mock_unique_user 
    password: mocksafepassword1234
```
---
`> Response:`

_If username not found in database:_

__400 Bad Request__

`> Body:`
```
    Username not found!
```
----
_Invalid password:_

__400 Bad Request__

`> Body:`
```
    Password doesn't match!
```
---
_Succesful:_

__200 OK__

`> Body:`
```json
{
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODMwMjE1OTIsImN1c3RvbSI6eyJ1c2VybmFtZSI6Im5vdHNvZmluYWxib3NzIiwibmFtZSI6IkZpbmFsIEJvc3MifX0.V4GK6d7HOYoSRvmlehPzH3LFcINXiErEWMLnVzlFO1Q",
    "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODM2MjE1NDIsImlkIjp7InVzZXJuYW1lIjoibm90c29maW5hbGJvc3MifX0.k0sP42jdMXCLlOIGHhoTlq-E7MMHAUSfi1U-ETw5zGs"
}
```

#### `REGISTER USER REQUEST`
---
`> Request-type:` _POST_

`> Url`: http://127.0.0.1:3000/user/register

`> Content-type:` __x-www-form-urlencoded__

`> Body:`
```
    username: mock_unique_user
    email: mock@mockmail.com
    password: mocksafepassword1234
```

---
`> Response:`

_If username/email already exist in database:_

__400 Bad Request__

`> Body:`
```
    Username or Email already in use!
```

----

_Succesful:_

__200 OK__

#### `VALIDATE USER REQUEST`
---
`> Request-type:` _GET_

`> Url`: http://127.0.0.1:3000/user/validate

`> Headers:`
```
Authorization: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODMwMjE1OTIsImN1c3RvbSI6eyJ1c2VybmFtZSI6Im5vdHNvZmluYWxib3NzIiwibmFtZSI6IkZpbmFsIEJvc3MifX0.V4GK6d7HOYoSRvmlehPzH3LFcINXiErEWMLnVzlFO1Q
```

Example using `curl`:
```bash
curl --header "Authorization: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODMwMTcxMjEsImN1c3RvbSI6eyJ1c2VybmFtZSI6Im5vdHNvZmluYWxib3NzIiwibmFtZSI6IkZpbmFsIEJvc3MifX0.4o7jVms9HzEUqwtF9FC-8HUS5aWmJOSoDuH_M2I0YPQ" -X GET http://127.0.0.1:3000/user/validate
```
---
`> Response:`

_If access token has timed out:_

__400 Bad Request__

`> Body:`
```
    Token timed out!
```

----
_If access token is invalid:_

__400 Bad Request__

`> Body:`
```
    Invalid token!
```

----
_Succesful:_

__200 OK__

`> Body:`
```json
{
    "username": "mockusername",
    "name": "John Doe"
}
```
### `SILENT REFRESH REQUEST`
---
`> Request-type:` _POST_

`> Url`: http://127.0.0.1:3000/user/refresh

`> Content-type:` __x-www-form-urlencoded__

`> Body:`
```
    token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODMwMTcxMjEsImN1c3RvbSI6eyJ1c2VybmFtZSI6Im5vdHNvZmluYWxib3NzIiwibmFtZSI6IkZpbmFsIEJvc3MifX0.4o7jVms9HzEUqwtF9FC-8HUS5aWmJOSoDuH_M2I0YPQ
```
---
`> Response:`

_If refresh token is invalid:_

__400 Bad Request__

`> Body:`
```json
{
    "message": "Refresh Token doesn't exist. Please redirect user to login."
}
```

----
_Succesful:_

__200 OK__

`> Body:`
```json
{
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2ODMwMjI5NDMsImN1c3RvbSI6eyJ1c2VybmFtZSI6Im5vdHNvZmluYWxib3NzIiwibmFtZSI6IkZpbmFsIEJvc3MifX0.TPI7JBS9YMdFrpMHPJ_D0abrpKLDYtcLF1ImBNygoHo"
}
```
