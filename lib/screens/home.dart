import 'package:flutter/material.dart';
import 'package:mail_app/types/message_update.dart';
import 'package:webview_windows/webview_windows.dart';

import 'package:mail_app/mail-client/enough_mail.dart';
import 'package:mail_app/services/overlay_builder.dart';
import 'package:mail_app/services/local_settings.dart';
import 'package:mail_app/services/inbox_service.dart';
import 'package:mail_app/utils/local_file_store.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/add_account.dart';
import 'package:mail_app/widgets/vertical_split.dart';
import 'package:mail_app/widgets/mailbox/mailbox_list.dart';
import 'package:mail_app/widgets/mailbox/mailbox_header.dart';
import 'package:mail_app/widgets/inbox/inbox_list.dart';
import 'package:mail_app/widgets/message/message_content.dart';
import 'package:mail_app/widgets/message/message_control_bar.dart';

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
  late WebviewController _messageWebviewController;
  late OverlayBuilder _overlayBuilder;

  int _activeID = 0;
  double _messageListPosition = 0;
  MimeMessage message = MimeMessage();

  @override
  void initState() {
    super.initState();

    _fileStore = widget.fileStore;
    _localSettings = widget.localSettings;
    _inboxService = widget.inboxService;
    _messageWebviewController = widget.messageWebviewController;
  }

  void _updateActiveID(int idx) {
    if (_activeID == idx) return;

    setState(() {
      _activeID = idx;
      message = _inboxService.getMessages().isNotEmpty
          ? _inboxService.getMessages()[_activeID]
          : MimeMessage();

      if (!message.isSeen) {
        _readMessage();
      }
    });
  }

  void _readMessage() async {
    await Future.delayed(const Duration(seconds: 2), () {});

    MimeMessage message = _inboxService.getMessages()[_activeID];

    if (_inboxService
        .currentClient()
        .getUnseenMessages()
        .toList()
        .contains(MessageSequence.fromMessage(message).toList().first)) {
      await _inboxService.currentClient().markMessage(
          _inboxService.getMessages()[_activeID], MessageUpdate.seen);
    }

    setState(() {
      message = _inboxService.getMessages()[_activeID];
    });
  }

  void _updateMessageListPosition(double position) {
    _messageListPosition = position;
  }

  Future<void> _updateMessageList(
      String email, String mailboxPath, String mailboxTitle) async {
    if (email == _inboxService.currentClient().getEmail() &&
        mailboxPath == _inboxService.currentClient().getCurrentMailboxPath()) {
      return;
    }

    _inboxService.updateLocalMailbox(email, mailboxPath);
    _updateMessageListPosition(0);

    setState(() {
      _activeID = 0;
      message = _inboxService.getMessages().isNotEmpty
          ? _inboxService.getMessages()[0]
          : MimeMessage();
    });
  }

  Future<void> _refreshAll() async {
    await _inboxService.updateInbox();
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

  void _markMessage(MessageUpdate messageUpdate) async {
    await _inboxService
        .currentClient()
        .markMessage(_inboxService.getMessages()[_activeID], messageUpdate);

    setState(() {
      message = _inboxService.getMessages()[_activeID];
    });
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
              child: MailboxList(
                  mailboxTree: _inboxService.getMailboxTree(),
                  updateMessageList: _updateMessageList,
                  activeMailbox: {
                    'email': _inboxService.currentClient().getEmail(),
                    'path':
                        _inboxService.currentClient().getCurrentMailboxPath(),
                  },
                  header: MailboxHeader(
                    composeMessage: _composeMessage,
                    addMailAccount: _addMailAccount,
                  ),
                  key: UniqueKey()),
            ),
            middle: SizedBox(
              height: double.infinity,
              child: MessageList(
                messages: _inboxService.getMessages(),
                unseenMessages:
                    _inboxService.currentClient().getUnseenMessages(),
                mailboxTitle:
                    _inboxService.currentClient().getCurrentMailboxTitle(),
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
                    markMessage: _markMessage,
                    reply: _reply,
                    replyAll: _replyAll,
                    share: _share,
                    key: UniqueKey(),
                  ),
                  Expanded(
                    child: MessageContent(
                      key: UniqueKey(),
                      message: _inboxService.getMessages().length > _activeID
                          ? message
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
