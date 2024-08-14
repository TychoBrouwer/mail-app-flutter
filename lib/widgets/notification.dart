import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';

import 'package:mail_app/services/overlay_builder.dart';
import 'package:mail_app/types/project_colors.dart';

class NotificationInfo {
  final String message;
  final bool showLoader;
  final int idx;

  double turns = 0;
  bool finished = false;

  NotificationInfo({
    required this.message,
    required this.showLoader,
    required this.idx,
  });
}

class NotificationManager {
  late OverlayBuilder _overlayBuilder;

  final Map<int, NotificationInfo?> _notifications = {};

  int _ovelayIdx = 1;

  void addOverlay(String message, bool showLoader) {
    _notifications[_ovelayIdx] = NotificationInfo(
      message: message,
      showLoader: showLoader,
      idx: _ovelayIdx,
    );

    _overlayBuilder.insertOverlay(
      messageBuild(_notifications[_ovelayIdx]!),
      _ovelayIdx,
    );

    if (showLoader) {
      _autoRemoveOverlay(_ovelayIdx);
    } else {
      _refreshRotate(_notifications[_ovelayIdx]!);
    }

    _ovelayIdx += 1;
  }

  void loadingFinished(int idx) {
    _notifications[idx]?.finished = true;
    _overlayBuilder.removeOverlay(idx);
  }

  void _autoRemoveOverlay(idx) async {
    await Future.delayed(const Duration(seconds: 2));

    _overlayBuilder.removeOverlay(idx);
    _notifications.remove(idx);
  }

  void _refreshRotate(NotificationInfo notification) async {
    // setState(() {
    notification.turns += 1;
    // });

    await Future.delayed(const Duration(seconds: 1), () {});

    if (!notification.finished) {
      _refreshRotate(notification);
    } else {
      _notifications.remove(notification.idx);
    }
  }

  Widget messageBuild(NotificationInfo notification) {
    return Container(
      key: ValueKey(notification.idx),
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
        color: ProjectColors.background(true),
        borderRadius: BorderRadius.circular(5),
      ),
      child: Row(
        children: [
          if (notification.showLoader)
            AnimatedRotation(
              alignment: Alignment.center,
              turns: notification.turns,
              duration: const Duration(seconds: 1),
              child: SvgPicture.asset(
                'assets/icons/arrows-rotate.svg',
                color: ProjectColors.text(true),
                width: 20,
                height: 20,
              ),
            ),
          SizedBox(
            width: 150,
            child: Text(
              notification.message,
              style: TextStyle(
                color: ProjectColors.text(true),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
