class MailAccount {
  late int sessionId;
  late String username;
  late String address;
  late int port;

  MailAccount(
    this.sessionId,
    this.username,
    this.address,
    this.port,
  );

  factory MailAccount.fromJson(Map<String, dynamic> data) {
    final sessionId = data['id'] as int;
    final username = data['username'] as String;
    final address = data['address'] as String;
    final port = data['port'] as int;

    return MailAccount(sessionId, username, address, port);
  }
}
