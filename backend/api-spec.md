# API spec

## LOGIN

login to an IMAP server and create a session.

/login

- `username` (string): The username of the user
- `password` (string): The password of the user
- `address` (string): The address of the user
- `port` (int): The port of the user

```json
{
    "success": true|false,
    "message": "message",
    "data": {                         // session id of new session
      "session_id": 1
    }
}
```

## LOGOUT

Logout from the IMAP server and delete the session.

/logout

- `session_id` (int): The session id of the user

```json
{
    "success": true|false,
    "message": "message"
}
```

## GET_SESSIONS

Get all the logged in IMAP sessions

/get_sessions

```json
{
    "success": true|false,
    "message": "message",
    "data": [                         // list of connected sessions
      {
        "session_id": 1,
        "username": "username",
        "address": "address",
        "port": 1
      }
    ]
}
```

## GET_MAILBOXES

Get all the mailbox paths of a session.

/get_mailboxes

- `session_id` (int): The session id of the user

```json
{
    "success": true|false,
    "message": "message",
    "data": [                         // list of mailbox paths
      "mailbox_path"
    ]
}
```

## GET_MESSAGES

Get a message from a mailbox using the message uids from the local database only.

/get_messages

- `session_id` (int): The session id of the user
- `mailbox_path` (string): The mailbox path
- `message_uids` (comma separated list): The uids of the messages

```json
{
  "success": true|false,
  "message": "message",
  "data": [                           // list of messages
    {
      "uid": 1,
      "sequence_id": 1,
      "message_id": "server message id",
      "subject": "subject",
      "from": [
        {
          "name": "Google",
          "mailbox": "no-reply",
          "host": "accounts.google.com"
        }
      ],
      "sender": [],                   // same object as from
      "to": [],                       // same object as from
      "cc": [],                       // same object as from
      "bcc": [],                      // same object as from
      "reply_to": [],                 // same object as from
      "in_reply_to": "email string",
      "delivered_to": "email string",
      "date": 1722093349000,
      "received": 1722093350000,
      "flags": ["Seen", "Flagged"],
      "html": "base64 encoded html",
      "text": "base64 encoded text"
    }
  ]
}
```

## GET_MESSAGES_SORTED

get messages from a mailbox sorted on time with indexes\
calculated from the start and end indexes of the index numbers.\
Messages are retrieved from the local database only.

/get_messages_sorted

- `session_id` (int): The session id of the user
- `mailbox_path` (string): The mailbox path
- `start` (int): The start index of the messages
- `end` (int): The end index of the messages

```json
{
  "success": true,
  "message": "message",
  "data": [                           // list of messages
    {
      "uid": 1,
      "sequence_id": 1,
      "message_id": "server message id",
      "subject": "subject",
      "from": [
        {
          "name": "Google",
          "mailbox": "no-reply",
          "host": "accounts.google.com"
        }
      ],
      "sender": [],                   // same object as from
      "to": [],                       // same object as from
      "cc": [],                       // same object as from
      "bcc": [],                      // same object as from
      "reply_to": [],                 // same object as from
      "in_reply_to": "email string",
      "delivered_to": "email string",
      "date": 1722093349000,
      "received": 1722093350000,
      "flags": ["Seen", "Flagged"],
      "html": "base64 encoded html",
      "text": "base64 encoded text"
    }
  ]
}
```

## UPDATE_MAILBOX

Update the mailbox of a session from the IMAP server.\

Algorithm:

1. select command to get exists (total number of messages)
2. fetch message with sequence id `exists` to get the uid
3. check if message in local database with sequence id `exists` has the same uid
4. if not, message is moved/deleted/added in the mailbox

    - fetch 'UID' for 10 at the time until sequence id and uid match
    - if message in local database update the sequence id and remove sequence id\
        from message in the same mailbox with the same sequence id
    - if message not in local database add message to local database

5. always, fetch with 'FLAGS' of all messages in the mailbox to update flags

/update_mailbox

- `session_id` (int): The session id of the user
- `mailbox_path` (string): The mailbox path

```json
{
  "success": true|false,
  "message": "message",
  "data": [                           // list of changed messages
    {
      "uid": 1,
      "sequence_id": 1,
      "message_id": "server message id",
      "subject": "subject",
      "from": [
        {
          "name": "Google",
          "mailbox": "no-reply",
          "host": "accounts.google.com"
        }
      ],
      "sender": [],                   // same object as from
      "to": [],                       // same object as from
      "cc": [],                       // same object as from
      "bcc": [],                      // same object as from
      "reply_to": [],                 // same object as from
      "in_reply_to": "email string",
      "delivered_to": "email string",
      "date": 1722093349000,
      "received": 1722093350000,
      "flags": ["Seen", "Flagged"],
      "html": "base64 encoded html",
      "text": "base64 encoded text"
    }
  ]
}
```

## MODIFY_FLAGS

Modify the flags of a message in a mailbox using the message uid.

/modify_flags

- `session_id` (int): The session id of the user
- `mailbox_path` (string): The mailbox path
- `message_uid` (int): The uid of the message
- `flags` (comma separated list): The flags to modify (e.g. "Seen,Flagged,Deleted")
- `add` (bool): If the flags should be added or removed

```json
{
  "success": true|false,
  "message": "message",
  "data": [                           // list of all flags
    "Seen",
    "Flagged"
  ]
}
```

## MOVE_MESSAGE

Move a message from one mailbox to another using the message uid.\
The message will be copied to the destination mailbox and deleted from\
the source mailbox using the IMAP move command.

/move_message

- `session_id` (int): The session id of the user
- `message_uid` (int): The uid of the message
- `mailbox_path_dest` (string): The destination mailbox path

```json
{
  "success": true|false,
  "message": "message",
  "data": "mailbox_path_dest"         // destination mailbox path
}
```
