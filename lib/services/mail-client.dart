import 'package:enough_mail/enough_mail.dart';

class MailClient {
  final ImapClient _client = ImapClient(isLogEnabled: false);

  late Map<String, List<MimeMessage>> _messages;
  late List<Mailbox> _mailBoxes;
  late Mailbox currentMailbox;

  List<MimeMessage> getMessages(Mailbox mailbox) {
    return _messages[mailbox.encodedPath]!;
  }

  MimeMessage getMessageFromIdx(idx) {
    if (_messages[currentMailbox] != null) {
      return _messages[currentMailbox]![idx];
    }

    return MimeMessage();
  }

  Future<bool> connect(
      {String email = 'test1928346534@gmail.com',
      String password = 'xsccljyfbfrgvtjw'}) async {
    try {
      await _client.connectToServer('imap.gmail.com', 993, isSecure: true);
      await _client.login(email, password);
      currentMailbox = await _client.selectInbox();
      await updateMailBoxes();

      return true;
    } on ImapException catch (e) {
      print(e);
    }

    return false;
  }

  Future<bool> disconnect() async {
    try {
      await _client.logout();

      return true;
    } on ImapException catch (e) {
      print(e);
    }

    return false;
  }

  Future<bool> selectMailbox(idx) async {
    try {
      currentMailbox = await _client.selectMailbox(_mailBoxes[idx]);

      print(currentMailbox);

      return true;
    } on ImapException catch (e) {
      print(e);
    }

    return false;
  }

  // Future<void> discoverExample() async {
  //   var email = 't.brouwer1@student.tue.nl';
  //   var config = await Discover.discover(email, isLogEnabled: false);
  //   if (config == null) {
  //   } else {
  //     for (var provider in config.emailProviders!) {}
  //   }
  // }

  // Future<bool> imapExample() async {
  //   try {
  //     await _client.selectInbox();
  //     // fetch 10 most recent messages:
  //     final fetchResult = await _client.fetchRecentMessages(
  //         messageCount: 100, criteria: 'BODY[]');

  //     _messages["INBOX"] = fetchResult.messages;

  //     return true;
  //   } on ImapException catch (e) {
  //     print(e);
  //   }

  //   return false;
  // }

  Future<bool> updateMailboxMessages(Mailbox mailBox) async {
    try {
      await _client.selectMailbox(mailBox);
      // fetch 10 most recent messages:
      final fetchResult = await _client.fetchRecentMessages(
          messageCount: 100, criteria: 'BODY[]');

      _messages[mailBox.encodedName] = fetchResult.messages;

      return true;
    } on ImapException catch (e) {
      print(e);
    }

    return false;
  }

  Future<List<Mailbox>> updateMailBoxes() async {
    _mailBoxes = await _client.listMailboxes(recursive: true);

    return _mailBoxes;
  }

  List<Mailbox> getMailBoxes() {
    return _mailBoxes;
  }

  List<String> getMailBoxNames() {
    return _mailBoxes.map((box) => box.encodedName).toList();
  }
}
