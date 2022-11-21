import '../mail-client/enough_mail.dart';

import './mail-service.dart';

class InboxService {
  final Map<String, CustomMailClient> _mailClients = {};

  late String _currentEmail;

  CustomMailClient newClient(
    String email,
    String password,
    String imapServer,
    int imapPort,
    String smtpServer,
    int smtpPort,
  ) {
    _mailClients[email] = CustomMailClient()..connect(email, password);
    _currentEmail = email;

    return _mailClients[email]!;
  }

  CustomMailClient clientFromEmail(String email) {
    return _mailClients[email] ?? CustomMailClient();
  }

  CustomMailClient currentClient() {
    return _mailClients[_currentEmail] ?? CustomMailClient();
  }

  List<MimeMessage> getMessages() {
    return _mailClients[_currentEmail]?.getMessages() ?? [];
  }

  Future<List<bool>> clientsConnected() {
    List<Future<bool>> clientConnections = [];

    _mailClients.forEach((email, client) {
      clientConnections.add(client.connected());
    });

    return Future.wait(clientConnections);
  }

  void updateLocalMailbox(String email, String mailboxPath) {
    _mailClients[email]!.selectLocalMailbox(mailboxPath);
  }

  Future<void> updateMailList(String email, String mailboxPath) async {
    await clientsConnected();

    if (_mailClients[email] == null) return;

    await _mailClients[email]!.updateMailboxFromPath(mailboxPath);
  }

  Map<String, CustomMailClient> connectedClients() {
    return Map.from(_mailClients)
      ..removeWhere((email, client) => client.isConnected() == false);
  }

  Map<String, CustomMailClient> clients() {
    return _mailClients;
  }

  List<String> clientEmails() {
    return _mailClients.keys.toList(growable: false);
  }

  Future<void> updateInbox() async {
    List<Future<void>> clientUpdates = [];

    _mailClients.forEach((email, client) {
      clientUpdates.add(client.updateMailBoxes());
    });

    await Future.wait(clientUpdates);
  }

  Map<String, List<MailboxInfo>> accountsTree() {
    Map<String, List<MailboxInfo>> accountsTree = {};

    _mailClients.forEach((email, client) {
      accountsTree[email] = client
          .getMailBoxes()
          .where((mailbox) => !RegExp(r'\[.*\]').hasMatch(mailbox.encodedName))
          .map((mailbox) => MailboxInfo(
                mailbox.encodedName == 'INBOX' ? email : mailbox.encodedName,
                mailbox.encodedPath,
                RegExp(r'\[.*\]').hasMatch(mailbox.encodedPath),
              ))
          .toList(growable: false);
    });

    return accountsTree;
  }
}

class MailboxInfo {
  final String display;
  final String path;
  final bool indent;

  MailboxInfo(this.display, this.path, this.indent);
}
