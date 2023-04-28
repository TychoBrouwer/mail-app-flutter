import 'dart:async';

import 'package:mail_app/mail-client/enough_mail.dart';
import 'package:mail_app/types/message_update.dart';
import 'package:mail_app/utils/wait_until.dart';

class CustomMailClient {
  final ImapClient _client = ImapClient(isLogEnabled: false);

  final Map<String, List<MimeMessage>> _messages = {};
  final Map<String, MessageSequence> _messagesUnseen = {};
  late List<Mailbox> _mailBoxes;
  late Mailbox _currentMailbox;
  late String _email;

  bool _connected = false;

  List<MimeMessage> getMessages() {
    return _messages[_currentMailbox.encodedPath]!;
  }

  MessageSequence getUnseenMesssages() {
    return _messagesUnseen[_currentMailbox.encodedPath]!;
  }

  MimeMessage getMessageFromIdx(int idx) {
    if (_messages.containsKey(_currentMailbox.encodedPath) &&
        _messages[_currentMailbox.encodedPath]!.isNotEmpty) {
      return _messages[_currentMailbox.encodedPath]![idx];
    } else {
      return MimeMessage();
    }
  }

  Future<bool> connected() async {
    return await waitUntil(() => _connected);
  }

  bool isConnected() {
    return _connected;
  }

  String getEmail() {
    return _email;
  }

  String getCurrentMailboxPath() {
    return _currentMailbox.encodedPath;
  }

  String getCurrentMailboxTitle() {
    return _currentMailbox.encodedName == 'INBOX'
        ? _email
        : _currentMailbox.encodedName;
  }

  Future<bool> connect(
      String email, String password, String imapAddress, int imapPort) async {
    try {
      _email = email;

      await _client.connectToServer(imapAddress, imapPort, isSecure: true);
      await _client.login(email, password);
      _currentMailbox = await _client.selectInbox();

      // handle fetching cached/ saved messages instead of next line
      await updateMailBoxes();

      _connected = true;
    } on ImapException catch (e) {
      print(e);
    }

    return _connected;
  }

  Future<bool> disconnect() async {
    try {
      await _client.logout();

      _connected = false;
    } on ImapException catch (e) {
      print(e);
    }

    return _connected;
  }

  void selectLocalMailbox(String mailboxPath) {
    _currentMailbox = getMailboxFromPath(mailboxPath);
  }

  Future<void> selectMailbox(String mailboxPath) async {
    try {
      await _client.selectMailbox(getMailboxFromPath(mailboxPath));
    } on ImapException catch (e) {
      print(e);
    }
  }

  // Future<void> updateMailboxFromPath(String mailboxPath) async {
  //   try {
  //     await _client.selectMailbox(getMailboxFromPath(mailboxPath));

  //     // final SearchImapResult sequenceFetch =
  //     //     await _client.searchMessages(searchCriteria: 'ALL');

  //     // if (sequenceFetch.matchingSequence != null &&
  //     //     sequenceFetch.matchingSequence!.isNotEmpty) {
  //     final FetchImapResult fetchResult =
  //         await _client.fetchMessages(MessageSequence.fromAll(), 'BODY.PEEK[]');

  //     _messages[mailboxPath] = fetchResult.messages;
  //     // }

  //     final SearchImapResult sequenceUnseenFetch =
  //         await _client.searchMessages(searchCriteria: 'UNSEEN');

  //     _messagesUnseen =
  //         sequenceUnseenFetch.matchingSequence ?? MessageSequence();

  //     print(_messagesUnseen.toList());
  //     print('test');
  //   } on ImapException catch (e) {
  //     print(e);
  //   }
  // }

  Future<void> updateMailbox(Mailbox? mailbox) async {
    try {
      if (mailbox == null) return;

      _client.selectMailbox(mailbox);
      final FetchImapResult fetchResult =
          await _client.fetchMessages(MessageSequence.fromAll(), 'BODY.PEEK[]');

      _messages[mailbox.encodedPath] = fetchResult.messages;

      final SearchImapResult sequenceUnseenFetch =
          await _client.searchMessages(searchCriteria: 'UNSEEN');

      _messagesUnseen[mailbox.encodedPath] =
          sequenceUnseenFetch.matchingSequence ?? MessageSequence();
    } on ImapException catch (e) {
      print(e);
    }
  }

  Future<MessageSequence?> unseenMessages() async {
    final sequenceFetch =
        await _client.searchMessages(searchCriteria: 'UNSEEN');

    return sequenceFetch.matchingSequence;
  }

  Future<void> updateMailBoxes() async {
    _mailBoxes = await _client.listMailboxes(recursive: true);

    for (var mailbox in _mailBoxes) {
      if (mailbox.encodedName.endsWith(']')) {
        continue;
      }

      if (!_messages.containsKey(mailbox.encodedPath)) {
        _messages[mailbox.encodedPath] = [];
      }

      await updateMailbox(mailbox);

      _client.selectMailbox(_currentMailbox);
    }
  }

  List<Mailbox> getMailBoxes() {
    return _mailBoxes;
  }

  List<String> getMailBoxNames() {
    return _mailBoxes.map((box) => box.encodedPath).toList(growable: false);
  }

  Mailbox getMailboxFromPath(String mailboxPath) {
    return _mailBoxes
        .where((mailbox) => mailbox.encodedPath == mailboxPath)
        .first;
  }

  String getMailboxFromFlag(MailboxFlag flag) {
    Iterable<Mailbox> mailboxes =
        _mailBoxes.where((mailbox) => mailbox.flags.contains(flag));

    if (mailboxes.isEmpty) {
      return _mailBoxes
          .where((mailbox) => mailbox.flags.contains(MailboxFlag.all))
          .first
          .encodedPath;
    }

    return mailboxes.first.encodedPath;
  }

  bool checkCurrentMaiboxFlag(MailboxFlag flag) {
    return _currentMailbox.flags.contains(flag);
  }

  Future<void> markMessage(
      MimeMessage message, MessageUpdate messageUpdate) async {
    final messageSequence = MessageSequence.fromMessage(message);

    print(message.fromEmail);

    MailboxFlag? flag;

    switch (messageUpdate) {
      case MessageUpdate.delete:
        flag = checkCurrentMaiboxFlag(MailboxFlag.trash)
            ? MailboxFlag.inbox
            : MailboxFlag.trash;
        break;
      case MessageUpdate.archive:
        flag = !checkCurrentMaiboxFlag(MailboxFlag.inbox)
            ? MailboxFlag.inbox
            : MailboxFlag.archive;
        break;
      case MessageUpdate.flag:
        flag = checkCurrentMaiboxFlag(MailboxFlag.flagged)
            ? MailboxFlag.inbox
            : MailboxFlag.flagged;
        break;
      default:
        break;
    }

    if (flag != null) {
      final newMailboxPath = getMailboxFromFlag(flag);

      _client.selectMailbox(_currentMailbox);
      final result = await _client.move(messageSequence,
          targetMailboxPath: newMailboxPath);

      print(result.responseCode);

      if (result.warnings.isEmpty) {
        _messages[newMailboxPath]!.add(message);
        _messages[_currentMailbox.encodedPath]!
            .removeWhere((e) => e == message);
      } else {
        // DISPLAY ERROR HERE
      }
    }
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
}
