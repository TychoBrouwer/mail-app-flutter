class NotificationInfo {
  final String message;
  final bool showLoader;
  final int idx;

  double turns = 1;
  bool finished = false;

  NotificationInfo({
    required this.message,
    required this.showLoader,
    required this.idx,
  });
}
