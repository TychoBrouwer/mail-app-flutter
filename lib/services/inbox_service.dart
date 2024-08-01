import 'dart:convert';

import 'package:mail_app/services/websocket_service.dart';
import 'package:mail_app/types/mail_account.dart';
import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/socket_message.dart';

class InboxService {
  late WebsocketService _websocketService;

  int? _activeSession;
  String? _activeMailbox;

  final List<MailAccount> _sessions = [];
  final List<MailboxInfo> _mailboxes = [];

  InboxService(WebsocketService websocketService) {
    _websocketService = websocketService;
  }

  void setActiveSessionId(int session) {
    _activeSession = session;
  }

  int? getActiveSessionId() {
    return _activeSession;
  }

  void setActiveMailbox(String mailbox) {
    _activeMailbox = mailbox;
  }

  String? getActiveSessionDisplay() {
    final sessions =
        _sessions.where((element) => element.sessionId == _activeSession);

    if (sessions.isEmpty) return '';

    return sessions.first.username;
  }

  String? getActiveMailbox() {
    return _activeMailbox;
  }

  String? getActiveMailboxDisplay() {
    final mailboxes =
        _mailboxes.where((element) => element.path == _activeMailbox);

    if (mailboxes.isEmpty) return '';

    return mailboxes.first.display;
  }

  Future<Map<int, List<MailboxInfo>>> getMailboxTree() async {
    Map<int, List<MailboxInfo>> mailboxTree = {};

    for (var session in _sessions) {
      final mailboxes = await getMailboxes(session: session.sessionId);

      mailboxTree[session.sessionId] = mailboxes;
    }

    return mailboxTree;
  }

  Future<int> newSession(
      String username, String password, String address, int port) async {
    String request =
        '/imap/login\r\nemail=$username\npassword=$password\naddress=$address\nport=$port';

    final response = await _websocketService.sendMessage(request);

    final decode = jsonDecode(response);
    final messageData = MessageData.fromJson(decode);

    if (!messageData.success) return -1;

    final session = messageData.data['id'] as int;

    _sessions.add(MailAccount(
      session,
      username,
      address,
      port,
    ));

    return session;
  }

  Future<List<MailAccount>> getSessions() async {
    String request = '/imap/sessions';

    final response = await _websocketService.sendMessage(request);

    final decode = jsonDecode(response);

    final messageData = MessageData.fromJson(decode);

    if (!messageData.success) return [];

    for (var session in (messageData.data as List)) {
      _sessions.add(MailAccount.fromJson(session));
    }

    return _sessions;
  }

  Future<List<MailboxInfo>> getMailboxes({int? session}) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }

    String request = '/imap/mailboxes\r\nsession_id=$_activeSession';

    final response = await _websocketService.sendMessage(request);

    final messageData = MessageData.fromJson(jsonDecode(response));

    if (!messageData.success) return [];

    _mailboxes.clear();

    for (var mailbox in (messageData.data as List)) {
      if ((mailbox as String).endsWith(']')) continue;
      _mailboxes.add(MailboxInfo.fromJson(mailbox));
    }

    return _mailboxes;
  }

  Future<List<Message>> getMessages(
      {int? session,
      String? mailbox,
      required int start,
      required int end}) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }
    if (mailbox == null) {
      mailbox = _activeMailbox;

      if (_activeMailbox == null) return [];
    }

    String request =
        '/imap/messages\r\nsession_id=$_activeSession\nmailbox=$_activeMailbox\nstart=$start\nend=$end';

    final response = await _websocketService.sendMessage(request);
    final messageData = MessageData.fromJson(jsonDecode(response));

    if (!messageData.success) return [];

    final List<Message> messages = [];
    for (var message in (messageData.data as List)) {
      messages.add(Message.fromJson(message));
    }

    return messages;
  }

  // Future<String> getMessage(int messageUid) async {
  //   String request =
  //       '/imap/message\r\nsession_id=$_activeSession\nmailbox=$_activeMailbox\nmessage_uid=$messageUid';

  //   final response = await _websocketService.sendMessage(request);
  //   return response;
  // }
}
