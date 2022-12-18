class MailAccount {
  late String email;
  late String password;
  late String imapAddress;
  late int imapPort;
  late String smtpAddress;
  late int smtpPort;

  MailAccount(
    this.email,
    this.password,
    this.imapAddress,
    this.imapPort,
    this.smtpAddress,
    this.smtpPort,
  );

  accountJson() {
    return {
      'email': email,
      'password': password,
      'imapAddress': imapAddress,
      'imapPort': imapPort,
      'smtpAddress': smtpAddress,
      'smtpPort': smtpPort,
    };
  }
}
