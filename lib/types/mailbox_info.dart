class MailboxInfo {
  final String display;
  final String path;
  final bool indent;

  MailboxInfo(this.display, this.path, this.indent);

  factory MailboxInfo.fromJson(String data, String username) {
    final path = data;

    var display = path.split('/').last.replaceAll('[', '').replaceAll(']', '');
    if (display == 'INBOX') {
      display = username;
    }

    final indent = path.split('/').length > 1;

    return MailboxInfo(display, path, indent);
  }
}
