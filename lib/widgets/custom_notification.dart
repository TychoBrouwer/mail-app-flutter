import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';
import 'package:mail_app/services/overlay_builder.dart';

import 'package:mail_app/types/notification_info.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';

class CustomNotification extends StatefulWidget {
  final NotificationInfo notification;
  final OverlayBuilder overlayBuilder;
  final Future? callback;

  const CustomNotification({
    super.key,
    required this.notification,
    required this.overlayBuilder,
    required this.callback,
  });

  @override
  CustomNotificationState createState() => CustomNotificationState();
}

class CustomNotificationState extends State<CustomNotification> {
  late NotificationInfo _notification;
  late OverlayBuilder _overlayBuilder;
  late Future? _callback;

  @override
  void initState() {
    super.initState();

    _notification = widget.notification;
    _overlayBuilder = widget.overlayBuilder;
    _callback = widget.callback;

    if (_notification.showLoader) {
      _refreshRotate();

      _awaitRemove();
    } else {
      _autoRemove();
    }
  }

  void _autoRemove() async {
    await Future.delayed(const Duration(seconds: 5), () {
      _notification.finished = true;
      _overlayBuilder.removeOverlay(_notification.idx);
    });
  }

  void _awaitRemove() async {
    await _callback;

    _notification.finished = true;
    _overlayBuilder.removeOverlay(_notification.idx);
  }

  void _refreshRotate() async {
    await Future.delayed(const Duration(seconds: 1), () {});

    if (_notification.finished) {
      return;
    }

    _refreshRotate();

    setState(() {
      _notification.turns += 1;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Positioned(
      right: 5,
      bottom: 5,
      child: Wrap(
        children: [
          Container(
            key: ValueKey(_notification.idx),
            padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
            decoration: BoxDecoration(
              color: ProjectColors.background(false),
              borderRadius: ProjectSizes.borderRadiusSmall,
            ),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                if (_notification.showLoader)
                  AnimatedRotation(
                    alignment: Alignment.center,
                    turns: _notification.turns,
                    duration: const Duration(seconds: 1),
                    child: SvgPicture.asset(
                      'assets/icons/arrows-rotate.svg',
                      color: ProjectColors.text(true),
                      width: 16,
                      height: 16,
                    ),
                  ),
                Container(
                  padding: const EdgeInsets.only(left: 10, right: 5, bottom: 2),
                  child: Text(
                    _notification.message,
                    style: TextStyle(
                      decoration: TextDecoration.none,
                      fontSize: ProjectSizes.fontSize,
                      fontWeight: FontWeight.w500,
                      color: ProjectColors.text(true),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
