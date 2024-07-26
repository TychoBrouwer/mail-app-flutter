# Database Design

| CONNECTIONS   |      |              |
|---------------|------|--------------|
| id            | PK   | INT          |
| username      |      | VARCHAR(500) |
| password      |      | VARCHAR(100) |
| address       |      | VARCHAR(500) |
| port          |      | INT          |
| updated_at    |      | DATETIME     |

| MAILBOXES           |      |              |
|---------------------|------|--------------|
| id                  | PK   | INT          |
| connection_id       | FK   | INT          |
| path                |      | VARCHAR(100) |
| updated_at          |      | DATETIME     |

| MESSAGES            |      |              |
|---------------------|------|--------------|
| uid                 | PK   | INT          |
| connection_id       | FK   | INT          |
| mailbox_id          | FK   | INT          |
| message_id          |      | VARCHAR(500) |
| subject             |      | VARCHAR(500) |
| from                |      | VARCHAR(500) |
| sender              |      | VARCHAR(500) |
| to                  |      | VARCHAR(500) |
| cc                  |      | VARCHAR(500) |
| bcc                 |      | VARCHAR(500) |
| reply_to            |      | VARCHAR(500) |
| in_reply_to         |      | VARCHAR(500) |
| delivered_to        |      | VARCHAR(500) |
| date                |      | DATETIME     |
| received            |      | DATETIME     |
| html                |      | TEXT         |
| text                |      | TEXT         |
| updated_at          |      | DATETIME     |

| ADDRESSES           |      |              |
|---------------------|------|--------------|
| id                  | PK   | INT          |
| address             |      | VARCHAR(500) |
| name                |      | VARCHAR(500) |
| type                |      | VARCHAR(100) |
