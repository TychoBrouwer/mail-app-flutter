enum SpecialMailboxType {
  archive,
  trash,
}

class MailAccount {
  late int sessionId;
  late String username;
  late String address;
  late int port;

  final Map<SpecialMailboxType, String> specialMailboxes = {};

  MailAccount(
    this.sessionId,
    this.username,
    this.address,
    this.port,
  );

  void setSpecialMailboxes(List<String> mailboxes) {
    specialMailboxes[SpecialMailboxType.archive] = mailboxes
        .where((e) =>
            e.toLowerCase().contains('archive') ||
            e.toLowerCase().contains('all'))
        .first;

    specialMailboxes[SpecialMailboxType.trash] = mailboxes
        .where((e) =>
            e.toLowerCase().contains('trash') ||
            e.toLowerCase().contains('delete'))
        .first;
  }

  factory MailAccount.fromJson(Map<String, dynamic> data) {
    final sessionId = data['session_id'] as int;
    final username = data['username'] as String;
    final address = data['address'] as String;
    final port = data['port'] as int;

    return MailAccount(sessionId, username, address, port);
  }
}
