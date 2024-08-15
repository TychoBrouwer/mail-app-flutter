# API spec

## LOGIN

login to an IMAP server and create a session.

/login

- `username` (string): The username of the user
- `password` (string): The password of the user
- `address` (string): The address of the user
- `port` (int): The port of the user

```jsonc
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

```jsonc
{
    "success": true|false,
    "message": "message"
}
```

## GET_SESSIONS

Get all the logged in IMAP sessions

/get_sessions

```jsonc
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

Get all the mailbox paths of a session from the local database only.

/get_mailboxes

- `session_id` (int): The session id of the user

```jsonc
{
    "success": true|false,
    "message": "message",
    "data": [                         // list of mailbox paths
      "INBOX",
      "mailbox_path"
    ]
}
```

## UPDATE_MAILBOXES

Update and get all the mailbox paths of a session from the IMAP server.

/update_mailboxes

- `session_id` (int): The session id of the user

```jsonc
{
    "success": true|false,
    "message": "message",
    "data": [                         // list of mailbox paths
      "INBOX",
      "mailbox_path"
    ]
}
```

## GET_MESSAGES_WITH_UIDS

Get a message from a mailbox using the message uids from the local database only.

/get_messages_with_uids

- `session_id` (int): The session id of the user
- `mailbox_path` (string): The mailbox path
- `message_uids` (comma separated list): The uids of the messages

```jsonc
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

```jsonc
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
3. check if message in local database with uid has the same sequence id
4. if not, message is moved/deleted/added in the mailbox

    - fetch 'UID' for 50 at the time until all sequence id and uid match
        - compare the sequence id of the fetched messages with the sequence id in the database
        - remove from database messages with sequence id of fetch where the sequence id is different
        - update sequence id of messages where the sequence id is different in the fetch
        - add message uids present in the fetched list but not in the database

5. always, fetch with 'FLAGS' of all messages in the mailbox to update flags

/update_mailbox

- `session_id` (int): The session id of the user
- `mailbox_path` (string): The mailbox path
- `quick` (bool?): will only fetch (20max) new messages and remove messages

```jsonc
{
  "success": true|false,
  "message": "message",
  "data": {
    "new_uids": [1, 2, 3],            // list of new uids
    "removed_uids": [1, 2, 3],        // list of removed uids (not in mailbox anymore)
    "changed_uids": [1, 2, 3],        // list of changed uids (flags changed)
  }
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

```jsonc
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

```jsonc
{
  "success": true|false,
  "message": "message",
  "data": "mailbox_path_dest"         // destination mailbox path
}
```
