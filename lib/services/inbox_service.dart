import '../mail-client/enough_mail.dart';

import 'mail_service.dart';
import 'package:mail_app/types/mailbox_info.dart';

class InboxService {
  final Map<String, CustomMailClient> _mailClients = {};

  late String _currentEmail;

  Future<CustomMailClient?> newClient(
    String email,
    String password,
    String imapAddress,
    int imapPort,
    String smtpServer,
    int smtpPort,
  ) async {
    if (_mailClients.containsKey(email)) return null;

    _mailClients[email] = CustomMailClient();
    final result = await _mailClients[email]!
        .connect(email, password, imapAddress, imapPort);

    if (result) {
      _currentEmail = email;

      return _mailClients[email]!;
    } else {
      _mailClients.remove(email);

      return null;
    }
  }

  CustomMailClient clientFromEmail(String email) {
    return _mailClients[email] ?? CustomMailClient();
  }

  CustomMailClient currentClient() {
    return _mailClients[_currentEmail]!;
  }

  List<MimeMessage> getMessages() {
    return _mailClients[_currentEmail]!.getMessages();
  }

  Future<List<bool>> clientsConnected() {
    List<Future<bool>> clientConnections = [];

    _mailClients.forEach((email, client) {
      clientConnections.add(client.connected());
    });

    return Future.wait(clientConnections);
  }

  void updateLocalMailbox(String email, String mailboxPath) {
    _currentEmail = email;
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

  Future<void> updateInbox() async {
    List<Future<void>> clientUpdates = [];

    _mailClients.forEach((email, client) {
      clientUpdates.add(client.updateMailBoxes());
    });

    await Future.wait(clientUpdates);
  }

  Map<String, List<MailboxInfo>> mailboxTree() {
    Map<String, List<MailboxInfo>> mailboxTree = {};

    _mailClients.forEach((email, client) {
      mailboxTree[email] = client
          .getMailBoxes()
          .where((mailbox) => !RegExp(r'\[.*\]').hasMatch(mailbox.encodedName))
          .map((mailbox) => MailboxInfo(
                mailbox.encodedName == 'INBOX' ? email : mailbox.encodedName,
                mailbox.encodedPath,
                RegExp(r'\/').hasMatch(mailbox.encodedPath),
              ))
          .toList(growable: false);
    });

    return mailboxTree;
  }
}
