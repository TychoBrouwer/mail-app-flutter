import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';
import 'package:mail_app/services/overlay_builder.dart';

import 'package:mail_app/types/notification_info.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';

class CustomNotification extends StatefulWidget {
  final NotificationInfo notification;
  final Future? callback;

  const CustomNotification({
    super.key,
    required this.notification,
    required this.callback,
  });

  @override
  CustomNotificationState createState() => CustomNotificationState();
}

class CustomNotificationState extends State<CustomNotification>
    with TickerProviderStateMixin {
  late NotificationInfo _notification;
  late Future? _callback;

  late final AnimationController _controller =
      AnimationController(vsync: this, duration: const Duration(seconds: 1))
        ..repeat();

  @override
  void initState() {
    super.initState();

    _notification = widget.notification;
    _callback = widget.callback;

    if (_notification.showLoader) {
      _awaitRemove();
    } else {
      _autoRemove();
    }
  }

  @override
  dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _autoRemove() async {
    await Future.delayed(const Duration(seconds: 5), () {
      OverlayBuilder().removeOverlay(_notification.idx);
    });
  }

  void _awaitRemove() async {
    await _callback;

    OverlayBuilder().removeOverlay(_notification.idx);
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
                  AnimatedBuilder(
                    animation: _controller,
                    builder: (BuildContext context, Widget? child) {
                      return Transform.rotate(
                        angle: _controller.value * 2 * 3.14,
                        child: child!,
                      );
                    },
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
