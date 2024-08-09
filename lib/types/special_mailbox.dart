enum SpecialMailboxType {
  archive,
  trash,
}

class SpecialMailbox {
  final SpecialMailboxType type;

  final mailboxPaths = {
    SpecialMailboxType.archive: '[INBOX]/All Mail',
    SpecialMailboxType.trash: '[INBOX]/Trash',
  };

  SpecialMailbox(this.type);

  String get path => mailboxPaths[type]!;
}

class TrashMailbox extends SpecialMailbox {
  TrashMailbox() : super(SpecialMailboxType.trash);
}

class ArchiveMailbox extends SpecialMailbox {
  ArchiveMailbox() : super(SpecialMailboxType.archive);
}
