import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

import 'package:mail_app/types/message.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';
import 'package:mail_app/widgets/custom_button.dart';
import 'package:mail_app/widgets/inbox/message_preview.dart';

class MessageList extends StatefulWidget {
  final List<Message> messages;
  final String mailboxTitle;
  final int activeID;
  final void Function(int) updateActiveID;
  final Future<void> Function() refreshAll;
  final void Function() loadMoreMessages;
  final PageStorageKey<int> messageListKey;

  const MessageList({
    super.key,
    required this.updateActiveID,
    required this.mailboxTitle,
    required this.messages,
    required this.activeID,
    required this.refreshAll,
    required this.loadMoreMessages,
    required this.messageListKey,
  });

  @override
  MessageListState createState() => MessageListState();
}

class MessageListState extends State<MessageList> {
  late List<Message> _messages;
  late String _mailboxTitle;
  late int _activeID;
  late void Function(int) _updateActiveID;
  late Future<void> Function() _refreshAll;
  late void Function() _loadMoreMessages;
  late PageStorageKey<int> _messageListKey;

  final ScrollController _listController = ScrollController();

  double turns = 0;
  bool rotatingFinished = true;
  bool refreshFinished = false;

  @override
  void initState() {
    super.initState();

    _messages = widget.messages;
    _mailboxTitle = widget.mailboxTitle;
    _activeID = widget.activeID;
    _updateActiveID = widget.updateActiveID;
    _refreshAll = widget.refreshAll;
    _loadMoreMessages = widget.loadMoreMessages;
    _messageListKey = widget.messageListKey;

    _listController.addListener(_loadMore);
  }

  void _refreshRotate() async {
    if (!rotatingFinished) return;

    rotatingFinished = false;

    setState(() {
      turns += 1;
    });

    await Future.delayed(const Duration(seconds: 1), () {});

    rotatingFinished = true;

    if (!refreshFinished) _refreshRotate();
  }

  @override
  void dispose() {
    _listController.dispose();
    super.dispose();
  }

  void _loadMore() async {
    final position = _listController.position;

    if (position.pixels == position.maxScrollExtent) {
      _loadMoreMessages();
    }
  }

  bool _getActive(int idx) {
    return _activeID == idx;
  }

  void resetScroll() {
    _listController.jumpTo(0);
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.only(left: 15, right: 15, top: 10),
          child: Row(
            children: [
              Expanded(
                child: Text(
                  RegExp(r'.*@.*\.').hasMatch(_mailboxTitle)
                      ? 'INBOX'
                      : _mailboxTitle.toUpperCase(),
                  textAlign: TextAlign.left,
                  style: TextStyle(
                    fontSize: ProjectSizes.fontSizeLarge,
                    fontWeight: FontWeight.bold,
                    color: ProjectColors.main(false),
                  ),
                ),
              ),
              CustomButton(
                onTap: () async => {
                  refreshFinished = false,
                  _refreshRotate(),
                  await _refreshAll(),
                  refreshFinished = true,
                },
                child: AnimatedRotation(
                  alignment: Alignment.center,
                  turns: turns,
                  duration: const Duration(seconds: 1),
                  child: Padding(
                    padding: const EdgeInsets.all(5),
                    child: SvgPicture.asset(
                      'assets/icons/arrows-rotate.svg',
                      colorFilter: ColorFilter.mode(
                          ProjectColors.main(false), BlendMode.srcIn),
                      width: 20,
                      height: 20,
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 15),
            child: ListView(
              key: _messageListKey,
              controller: _listController,
              padding: const EdgeInsets.only(bottom: 200),
              children: [
                for (int idx = 0; idx < _messages.length; idx++)
                  MailPreview(
                    message: _messages[idx],
                    idx: idx,
                    getActive: _getActive,
                    updateMessageID: _updateActiveID,
                    key: UniqueKey(),
                  ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}
