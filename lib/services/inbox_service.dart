import 'package:mail_app/services/http_service.dart';
import 'package:mail_app/types/http_request_path.dart';
import 'package:mail_app/types/mail_account.dart';
import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/message_flag.dart';
import 'package:mail_app/types/special_mailbox.dart';

class InboxService {
  int? _activeSession;
  String? _activeMailbox;

  final HttpService httpService = HttpService();

  final List<MailAccount> _sessions = [];
  final List<MailboxInfo> _mailboxes = [];

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

  String getSpecialMailbox(SpecialMailboxType type) {
    final session = _sessions[_activeSession!];

    return session.specialMailboxes[type] ?? '';
  }

  Future<int> newSession(
    String username,
    String password,
    String address,
    int port,
  ) async {
    final body = {
      'email': username,
      'password': password,
      'address': address,
      'port': port.toString(),
    };

    final messageData =
        await HttpService().sendRequest(HttpRequestPath.login, body);

    if (!messageData.success) return -1;

    final session = messageData.data['session_id'] as int;

    _sessions.add(MailAccount(
      session,
      username,
      address,
      port,
    ));

    return session;
  }

  Future<List<MailAccount>?> getSessions() async {
    final messageData =
        await httpService.sendRequest(HttpRequestPath.get_sessions, {});

    if (!messageData.success) return null;

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

    final body = {
      'session_id': session.toString(),
    };

    final messageData =
        await HttpService().sendRequest(HttpRequestPath.get_mailboxes, body);

    if (!messageData.success) return [];

    _mailboxes.clear();

    final mailboxes = (messageData.data as List).cast<String>();
    _sessions[session!].setSpecialMailboxes(mailboxes);

    for (var mailbox in mailboxes) {
      if (mailbox.endsWith(']')) continue;

      _mailboxes
          .add(MailboxInfo.fromJson(mailbox, getActiveSessionDisplay() ?? ''));
    }

    return _mailboxes;
  }

  Future<List<MailboxInfo>> updateMailboxes({int? session}) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }

    final body = {
      'session_id': session.toString(),
    };

    final messageData =
        await HttpService().sendRequest(HttpRequestPath.update_mailboxes, body);

    if (!messageData.success) return [];

    _mailboxes.clear();

    final mailboxes = (messageData.data as List).cast<String>();
    _sessions[session!].setSpecialMailboxes(mailboxes);

    for (var mailbox in mailboxes) {
      if (mailbox.endsWith(']')) continue;

      _mailboxes
          .add(MailboxInfo.fromJson(mailbox, getActiveSessionDisplay() ?? ''));
    }

    return _mailboxes;
  }

  Future<List<Message>> getMessages({
    int? session,
    String? mailbox,
    required int start,
    required int end,
  }) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }
    if (mailbox == null) {
      mailbox = _activeMailbox;

      if (_activeMailbox == null) return [];
    }

    final body = {
      'session_id': session.toString(),
      'mailbox_path': mailbox!,
      'start': start.toString(),
      'end': end.toString(),
    };

    final messageData = await HttpService()
        .sendRequest(HttpRequestPath.get_messages_sorted, body);

    if (!messageData.success) return [];

    print(messageData.data);

    final List<Message> messages = [];
    for (var message in (messageData.data as List)) {
      messages.add(Message.fromJson(message));
    }

    return messages;
  }

  Future<List<Message>> getMessagesWithUids({
    int? session,
    String? mailbox,
    required List<int> uids,
  }) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }
    if (mailbox == null) {
      mailbox = _activeMailbox;

      if (_activeMailbox == null) return [];
    }

    final body = {
      'session_id': session.toString(),
      'mailbox_path': mailbox!,
      'uids': uids.join(','),
    };

    final messageData = await HttpService()
        .sendRequest(HttpRequestPath.get_messages_with_uids, body);

    if (!messageData.success) return [];

    final data = messageData.data as List<dynamic>;
    final List<Message> messages = data.map((e) {
      final messageData = e as Map<String, dynamic>;

      return Message.fromJson(messageData);
    }).toList();

    return messages;
  }

  Future<List<MessageFlag>> modifyFlags({
    int? session,
    String? mailbox,
    required int messageUid,
    required List<MessageFlag> flags,
    required bool add,
  }) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }
    if (mailbox == null) {
      mailbox = _activeMailbox;

      if (_activeMailbox == null) return [];
    }

    final flagsString = flags.map((e) => e.name).join(',');
    final addString = add.toString();

    final body = {
      'session_id': session.toString(),
      'mailbox_path': mailbox!,
      'message_uid': messageUid.toString(),
      'flags': flagsString,
      'add': addString,
    };

    final messageData =
        await HttpService().sendRequest(HttpRequestPath.modify_flags, body);

    if (!messageData.success) return [];

    return messageFlagsFromJsonList(messageData.data);
  }

  Future<String> moveMessage({
    int? session,
    String? mailbox,
    required int messageUid,
    required String mailboxDest,
  }) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return '';
    }
    if (mailbox == null) {
      mailbox = _activeMailbox;

      if (_activeMailbox == null) return '';
    }

    final body = {
      'session_id': session.toString(),
      'mailbox_path': mailbox!,
      'message_uid': messageUid.toString(),
      'mailbox_path_dest': mailboxDest,
    };

    final messageData =
        await HttpService().sendRequest(HttpRequestPath.move_message, body);

    if (!messageData.success) return '';

    return messageData.data;
  }

  Future<List<List<int>>> updateInbox({
    int? session,
    String? mailbox,
  }) async {
    if (session == null) {
      session = _activeSession;

      if (_activeSession == null) return [];
    }
    if (mailbox == null) {
      mailbox = _activeMailbox;

      if (_activeMailbox == null) return [];
    }

    final body = {
      'session_id': session.toString(),
      'mailbox_path': mailbox!,
      'quick': 'true',
    };

    final messageData =
        await HttpService().sendRequest(HttpRequestPath.update_mailbox, body);

    if (!messageData.success) return [];

    final data = (messageData.data as Map<String, dynamic>);

    final newUids = (data['new_uids'] as List).map((e) => e as int).toList();
    final changedUids =
        (data['changed_uids'] as List).map((e) => e as int).toList();
    final removedUids =
        (data['removed_uids'] as List).map((e) => e as int).toList();

    return [newUids, changedUids, removedUids];
  }
}
