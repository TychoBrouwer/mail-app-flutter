import 'package:flutter/material.dart';
import 'package:mail_app/services/overlay_builder.dart';
import 'package:mail_app/utils/local_file_store.dart';
import 'package:mail_app/services/local_settings.dart';
import 'package:mail_app/types/mailbox_info.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/add_account.dart';
import 'package:mail_app/widgets/control_bar.dart';
import 'package:mail_app/widgets/mailbox_list.dart';
import 'package:mail_app/widgets/mailbox_header.dart';
import 'package:mail_app/widgets/message_content.dart';
import 'package:webview_windows/webview_windows.dart';

import '../mail-client/enough_mail.dart';
import '../services/inbox_service.dart';
import '../widgets/vertical_split.dart';
import '../widgets/message_list.dart';

class HomePage extends StatefulWidget {
  final LocalFileStore fileStore;
  final LocalSettings localSettings;
  final InboxService inboxService;
  final WebviewController messageWebviewController;

  const HomePage(
      {super.key,
      required this.fileStore,
      required this.localSettings,
      required this.inboxService,
      required this.messageWebviewController});

  @override
  HomePageState createState() => HomePageState();
}

class HomePageState extends State<HomePage> {
  late LocalFileStore _fileStore;
  late LocalSettings _localSettings;
  late InboxService _inboxService;
  late Map<String, String> _activeMailbox;
  late WebviewController _messageWebviewController;
  late OverlayBuilder _overlayBuilder;

  List<MimeMessage> _messages = [];
  Map<String, List<MailboxInfo>> _mailboxTree = {};
  int _activeID = 0;
  String _mailboxTitle = '';
  double _messageListPosition = 0;

  @override
  void initState() {
    super.initState();

    _fileStore = widget.fileStore;
    _localSettings = widget.localSettings;
    _inboxService = widget.inboxService;
    _messageWebviewController = widget.messageWebviewController;
    _activeMailbox = {
      'email': _inboxService.currentClient().getEmail(),
      'path': _inboxService.currentClient().getCurrentMailboxPath(),
    };
    _mailboxTitle = _inboxService.currentClient().getCurrentMailboxTitle();

    _setMessages();
    _updateInbox();
  }

  void _updateActiveID(int idx) {
    if (_activeID == idx) return;

    setState(() {
      _activeID = idx;
    });
  }

  void _updateActiveMailbox(String email, String path) {
    final newActive = {
      'email': email,
      'path': path,
    };

    if (_activeMailbox == newActive) return;

    setState(() {
      _activeMailbox = newActive;
    });
  }

  void _updateMessageListPosition(double position) {
    _messageListPosition = position;
  }

  void _setMessages() {
    final List<MimeMessage> messages = _inboxService.getMessages();

    messages.sort((a, b) => b
        .decodeDate()!
        .millisecondsSinceEpoch
        .compareTo(a.decodeDate()!.millisecondsSinceEpoch));

    setState(() {
      _messages = messages;
    });
  }

  void _updateInbox() {
    _inboxService.updateInbox();

    setState(() {
      _mailboxTree = _inboxService.mailboxTree();
    });
  }

  void _updateMessageList(
      String email, String mailboxPath, String mailboxTitle) {
    _activeID = 0;
    _mailboxTitle = mailboxTitle;
    _inboxService.updateLocalMailbox(email, mailboxPath);

    _setMessages();
  }

  Future<void> _refreshAll() async {
    await _inboxService.updateInbox();

    _setMessages();
  }

  void _addMailAccount() {
    _overlayBuilder.insertOverlay(AddAccount(
      inboxService: _inboxService,
      overlayBuilder: _overlayBuilder,
      localSettings: _localSettings,
    ));
  }

  Future<void> _composeMessage() async {
    print('composing a message');
  }

  Future<void> _archive() async {
    print('archive a message');
  }

  Future<void> _markImportant() async {
    print('mark as important');
  }

  Future<void> _markDeleted() async {
    print('mark as deleted');
  }

  Future<void> _markUnread() async {
    print('mark as unread');
  }

  Future<void> _reply() async {
    print('reply to message');
  }

  Future<void> _replyAll() async {
    print('reply to all message');
  }

  Future<void> _share() async {
    print('share message');
  }

  @override
  Widget build(BuildContext context) {
    _overlayBuilder = OverlayBuilder(context);

    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          color: ProjectColors.secondary(true),
        ),
        child: Center(
          child: VerticalSplitView(
            left: Container(
              decoration: BoxDecoration(
                border: Border(
                    right: BorderSide(color: ProjectColors.secondary(false))),
              ),
              height: double.infinity,
              child: InboxList(
                  mailboxTree: _mailboxTree,
                  updateMessageList: _updateMessageList,
                  activeMailbox: _activeMailbox,
                  updateActiveMailbox: _updateActiveMailbox,
                  header: MailboxHeader(
                    composeMessage: _composeMessage,
                    addMailAccount: _addMailAccount,
                  ),
                  key: UniqueKey()),
            ),
            middle: SizedBox(
              height: double.infinity,
              child: MessageList(
                messages: _messages,
                mailboxTitle: _mailboxTitle,
                activeID: _activeID,
                updateActiveID: _updateActiveID,
                refreshAll: _refreshAll,
                listPosition: _messageListPosition,
                updatePosition: _updateMessageListPosition,
                key: UniqueKey(),
              ),
            ),
            right: Container(
              decoration: BoxDecoration(
                border: Border(
                    left: BorderSide(color: ProjectColors.secondary(false))),
              ),
              height: double.infinity,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                mainAxisSize: MainAxisSize.min,
                children: [
                  ControlBar(
                    archive: _archive,
                    markImportant: _markImportant,
                    markDeleted: _markDeleted,
                    markUnread: _markUnread,
                    reply: _reply,
                    replyAll: _replyAll,
                    share: _share,
                    key: UniqueKey(),
                  ),
                  Expanded(
                    child: MessageContent(
                      key: UniqueKey(),
                      message: _messages.length > _activeID
                          ? _messages[_activeID]
                          : MimeMessage(),
                      controller: _messageWebviewController,
                    ),
                  ),
                ],
              ),
            ),
            ratio2: 0.25,
            minRatio2: 0.1,
            maxRatio2: 0.45,
            ratio1: 0.2,
            minRatio1: 0.1,
            maxRatio1: 0.25,
          ),
        ),
      ),
    );
  }
}


// [
//   "INBOX" exists: 3, highestModeSequence: 1589, flags: [MailboxFlag.hasNoChildren, MailboxFlag.inbox],
//   "[Gmail]" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasChildren, MailboxFlag.noSelect],
//   "[Gmail]/All Mail" exists: 0, highestModeSequence: null, flags: [MailboxFlag.all, MailboxFlag.hasNoChildren],
//   "[Gmail]/Drafts" exists: 0, highestModeSequence: null, flags: [MailboxFlag.drafts, MailboxFlag.hasNoChildren],
//   "[Gmail]/Important" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasNoChildren, MailboxFlag.flagged],
//   "[Gmail]/Sent Mail" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasNoChildren, MailboxFlag.sent],
//   "[Gmail]/Spam" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasNoChildren, MailboxFlag.junk],
//   "[Gmail]/Starred" exists: 0, highestModeSequence: null, flags: [MailboxFlag.flagged, MailboxFlag.hasNoChildren],
//   "[Gmail]/Trash" exists: 0, highestModeSequence: null, flags: [MailboxFlag.hasNoChildren, MailboxFlag.trash]
// ]
