class MailboxInfo {
  final String display;
  final String path;
  final bool indent;

  MailboxInfo(this.display, this.path, this.indent);

  factory MailboxInfo.fromJson(String data) {
    final path = data;

    var display = path.split('/').last.replaceAll('[', '').replaceAll(']', '');
    if (display == 'INBOX') {
      display = 'Inbox';
    }

    final indent = path.split('/').length > 1;

    return MailboxInfo(display, path, indent);
  }
}
