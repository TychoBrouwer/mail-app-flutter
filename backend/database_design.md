# Database Design

| CONNECTIONS   |      |              |      |
|---------------|------|--------------|------|
| id            | PK   | INT          |      |
| username      |      | VARCHAR(500) |      |
| password      |      | VARCHAR(100) |      |
| address       |      | VARCHAR(500) |      |
| port          |      | INT          |      |
| updated       |      | DATETIME     | NULL |

| MAILBOXES     |      |              |      |
|---------------|------|--------------|------|
| id            | PK   | INT          |      |
| connection_id | FK   | INT          |      |
| path          |      | VARCHAR(100) |      |
| updated       |      | DATETIME     | NULL |

| MESSAGES      |      |              |      |
|---------------|------|--------------|------|
| id            | PK   | INT          |      |
| mailbox_id    | FK   | INT          |      |
| connection_id | FK   | INT          |      |
| uid           |      | INT          |      |
| message_id    |      | VARCHAR(500) |      |
| subject       |      | VARCHAR(500) |      |
| from          |      | VARCHAR(500) |      |
| sender        |      | VARCHAR(500) | NULL |
| to            |      | VARCHAR(500) |      |
| cc            |      | VARCHAR(500) | NULL |
| bcc           |      | VARCHAR(500) | NULL |
| reply_to      |      | VARCHAR(500) | NULL |
| in_reply_to   |      | VARCHAR(500) | NULL |
| delivered_to  |      | VARCHAR(500) | NULL |
| date          |      | DATETIME     |      |
| received      |      | DATETIME     | NULL |
| html          |      | TEXT         | NULL |
| text          |      | TEXT         | NULL |
