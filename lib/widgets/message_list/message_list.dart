import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart' show ScrollDirection;

import '../../types/message.dart';
import '../../types/project_colors.dart';
import 'message_preview.dart';

class MessageList extends StatefulWidget {
  final List<Message> messages;
  final int activeID;
  final void Function(int) updateActiveID;
  final void Function() loadMoreMessages;
  final PageStorageKey<int> messageListKey;

  const MessageList({
    super.key,
    required this.updateActiveID,
    required this.messages,
    required this.activeID,
    required this.loadMoreMessages,
    required this.messageListKey,
  });

  @override
  MessageListState createState() => MessageListState();
}

class MessageListState extends State<MessageList> {
  late List<Message> _messages;
  late int _activeID;
  late void Function(int) _updateActiveID;
  late void Function() _loadMoreMessages;
  late PageStorageKey<int> _messageListKey;

  final ScrollController _scrollController = ScrollController();

  double turns = 0;
  bool rotatingFinished = true;
  bool refreshFinished = false;

  bool _scrollUpdated = false;

  @override
  void initState() {
    super.initState();

    _messages = widget.messages;
    _activeID = widget.activeID;
    _updateActiveID = widget.updateActiveID;
    _loadMoreMessages = widget.loadMoreMessages;
    _messageListKey = widget.messageListKey;

    _scrollController.addListener(_scrollUpdate);
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  void _scrollUpdate() async {
    final position = _scrollController.position;

    if (position.pixels >= position.maxScrollExtent - 500 &&
        !_scrollUpdated &&
        _scrollController.position.userScrollDirection ==
            ScrollDirection.reverse) {
      _loadMoreMessages();
      _scrollUpdated = true;
    }

    if (position.pixels < position.maxScrollExtent - 500) {
      _scrollUpdated = false;
    }
  }

  bool _getActive(int idx) {
    return _activeID == idx;
  }

  List<Widget> messageListBuilder() {
    List<Widget> messageListWidgets = [];

    for (var message in _messages) {
      messageListWidgets.add(
        Padding(
          padding: const EdgeInsets.only(right: 10, bottom: 3),
          child: MessagePreview(
            message: message,
            idx: _messages.indexOf(message),
            getActive: _getActive,
            updateMessageID: _updateActiveID,
            key: UniqueKey(),
          ),
        ),
      );
    }

    return messageListWidgets;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.only(left: 5, right: 5, top: 10),
      decoration: BoxDecoration(
        border: Border.symmetric(
          vertical: BorderSide(
            color: ProjectColors.border(false),
            width: 1,
          ),
        ),
      ),
      child: ListView(
        key: _messageListKey,
        controller: _scrollController,
        padding: const EdgeInsets.only(bottom: 200),
        children: messageListBuilder(),
      ),
    );
  }
}
