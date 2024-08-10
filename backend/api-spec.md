# API spec

## LOGIN

### Request - LOGIN

/login

### Arguments - LOGIN

- `username` (string): The username of the user
- `password` (string): The password of the user
- `address` (string): The address of the user
- `port` (int): The port of the user

### Response - LOGIN

```json
{
    "success": true|false,
    "message": "message",
    "data": {
      "session_id": 1
    }
}
```

## LOGOUT

### Request - LOGOUT

/logout

### Arguments - LOGOUT

- `session_id` (int): The session id of the user

### Response - LOGOUT

```json
{
    "success": true|false,
    "message": "message"
}
```

## GET_SESSIONS

### Request - GET_SESSIONS

/get_sessions

### Arguments - GET_SESSIONS

-

### Response - GET_SESSIONS

```json
{
    "success": true|false,
    "message": "message",
    "data": {
      "sessions": [
        {
          "session_id": 1,
          "username": "username",
          "address": "address",
          "port": 1
        }
      ]
    }
}
```

## GET_MAILBOXES

### Request - GET_MAILBOXES

/get_mailboxes

### Arguments - GET_MAILBOXES

- `session_id` (int): The session id of the user

### Response - GET_MAILBOXES

```json
{
    "success": true|false,
    "message": "message",
    "data": {
      "mailboxes": [
        {
          "mailbox_id": 1,
          "name": "name",
          "address": "address",
          "port": 1
        }
      ]
    }
}
```
