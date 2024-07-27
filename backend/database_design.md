# Database Design

| CONNECTIONS   |      |              |
|---------------|------|--------------|
| username      | PK   | VARCHAR(500) |
| password      |      | VARCHAR(500) |
| address       | PK   | VARCHAR(500) |
| port          |      | INT          |
| updated_at    |      | DATETIME     |

| MAILBOXES           |        |              |
|---------------------|--------|--------------|
| connection_username | FK, PK | INT          |
| connection_address  | FK, PK | INT          |
| path                | PK     | VARCHAR(500) |
| updated_at          |        | DATETIME     |

| MESSAGES            |        |              |
|---------------------|--------|--------------|
| uid                 | PK     | INT          |
| connection_username | FK, PK | INT          |
| connection_address  | FK, PK | INT          |
| mailbox_path        | FK, PK | INT          |
| message_id          |        | VARCHAR(500) |
| subject             |        | VARCHAR(500) |
| from                |        | VARCHAR(500) |
| sender              |        | VARCHAR(500) |
| to                  |        | VARCHAR(500) |
| cc                  |        | VARCHAR(500) |
| bcc                 |        | VARCHAR(500) |
| reply_to            |        | VARCHAR(500) |
| in_reply_to         |        | VARCHAR(500) |
| delivered_to        |        | VARCHAR(500) |
| date                |        | DATETIME     |
| received            |        | DATETIME     |
| html                |        | TEXT         |
| text                |        | TEXT         |
| updated_at          |        | DATETIME     |
