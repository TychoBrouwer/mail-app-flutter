import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:mail_app/services/inbox_service.dart';
import 'package:mail_app/services/local_settings.dart';
import 'package:mail_app/services/overlay_builder.dart';
import 'package:mail_app/types/mail_account.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/custom_button.dart';
import 'package:mail_app/widgets/custom_form_field.dart';
import 'package:email_validator/email_validator.dart';

class AddAccount extends StatefulWidget {
  final InboxService inboxService;
  final OverlayBuilder overlayBuilder;
  final LocalSettings localSettings;

  const AddAccount({
    super.key,
    required this.inboxService,
    required this.overlayBuilder,
    required this.localSettings,
  });

  @override
  AddAccountState createState() => AddAccountState();
}

class AddAccountState extends State<AddAccount> {
  late InboxService _inboxService;
  late OverlayBuilder _overlayBuilder;
  late LocalSettings _localSettings;

  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  bool loading = false;

  String? _email;
  String? _password;
  String? _smtpAddress;
  int? _smtpPort;
  String? _imapAddress;
  int? _imapPort;

  @override
  void initState() {
    super.initState();

    _inboxService = widget.inboxService;
    _overlayBuilder = widget.overlayBuilder;
    _localSettings = widget.localSettings;
  }

  void _cancel() {
    _overlayBuilder.removeOverlay();
  }

  Future<void> _confirm() async {
    loading = true;

    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();

      final mailAccount = MailAccount(
        _email!,
        _password!,
        _imapAddress!,
        _imapPort!,
        _smtpAddress!,
        _smtpPort!,
      );

      final connection = await _inboxService.newClient(
        mailAccount.email,
        mailAccount.password,
        mailAccount.imapAddress,
        mailAccount.imapPort,
        mailAccount.smtpAddress,
        mailAccount.smtpPort,
      );

      if (connection != null) {
        print('success');

        _localSettings.addAccount(mailAccount);
        _localSettings.saveSettings();

        _overlayBuilder.removeOverlay();
      } else {
        print('failed to add');
      }

      loading = false;
    }
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: MediaQuery.of(context).size.width,
      height: MediaQuery.of(context).size.height,
      child: Padding(
        padding: EdgeInsets.symmetric(
          horizontal: MediaQuery.of(context).size.width * 0.24,
          vertical: MediaQuery.of(context).size.height * 0.06,
        ),
        child: Container(
          decoration: BoxDecoration(
            border: Border.all(color: ProjectColors.main(false)),
            color: const Color.fromRGBO(33, 33, 33, 1),
            borderRadius: BorderRadius.circular(2),
          ),
          child: Form(
            key: _formKey,
            child: Padding(
              padding: EdgeInsets.all(MediaQuery.of(context).size.width * 0.05),
              child: ListView(
                // crossAxisAlignment: CrossAxisAlignment.start,
                children: <Widget>[
                  CustomFormField(
                    onSaved: (val) => _email = val,
                    labelText: 'Email address',
                    validator: (String? value) {
                      if (value == null || value.isEmpty) {
                        return 'Please enter your email';
                      } else if (!EmailValidator.validate(value)) {
                        return 'Please enter a valid email';
                      }
                      return null;
                    },
                  ),
                  CustomFormField(
                    onSaved: (val) => _password = val,
                    labelText: 'Password',
                    obscureText: true,
                    validator: (String? value) {
                      if (value == null || value.isEmpty) {
                        return 'Please enter your password';
                      }
                      return null;
                    },
                  ),
                  Row(
                    children: [
                      Flexible(
                        flex: 2,
                        child: CustomFormField(
                          onSaved: (val) => _smtpAddress = val,
                          labelText: 'SMTP address',
                          validator: (String? value) {
                            if (value == null || value.isEmpty) {
                              return 'Please enter an SMTP address';
                            }
                            return null;
                          },
                        ),
                      ),
                      Flexible(
                        child: CustomFormField(
                          onSaved: (val) =>
                              _smtpPort = (val != null) ? int.parse(val) : 0,
                          labelText: 'SMTP port',
                          keyboardType: TextInputType.number,
                          inputFormatters: [
                            FilteringTextInputFormatter.digitsOnly
                          ],
                          validator: (String? value) {
                            if (value == null || value.isEmpty) {
                              return 'Enter SMTP Port';
                            }
                            return null;
                          },
                        ),
                      ),
                    ],
                  ),
                  Row(
                    children: [
                      Flexible(
                        flex: 2,
                        child: CustomFormField(
                          onSaved: (val) => _imapAddress = val,
                          labelText: 'IMAP address',
                          validator: (String? value) {
                            if (value == null || value.isEmpty) {
                              return 'Please enter an IMAP address';
                            }
                            return null;
                          },
                        ),
                      ),
                      Flexible(
                        child: CustomFormField(
                          onSaved: (val) =>
                              _imapPort = (val != null) ? int.parse(val) : 0,
                          labelText: 'IMAP port',
                          keyboardType: TextInputType.number,
                          inputFormatters: [
                            FilteringTextInputFormatter.digitsOnly
                          ],
                          validator: (String? value) {
                            if (value == null || value.isEmpty) {
                              return 'Enter IMAP port';
                            }
                            return null;
                          },
                        ),
                      ),
                    ],
                  ),
                  Padding(
                    padding: const EdgeInsets.only(top: 20, left: 5),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      children: [
                        CustomButton(
                          onTap: _cancel,
                          child: Container(
                            padding: const EdgeInsets.symmetric(
                                horizontal: 15, vertical: 5),
                            child: Text(
                              'CANCEL',
                              style: TextStyle(
                                color: ProjectColors.main(false),
                                decoration: TextDecoration.none,
                                fontSize: 18,
                                fontWeight: FontWeight.normal,
                              ),
                            ),
                          ),
                        ),
                        CustomButton(
                          onTap: _confirm,
                          child: Container(
                            padding: const EdgeInsets.symmetric(
                                horizontal: 15, vertical: 5),
                            child: Text(
                              'CONFIRM',
                              textAlign: TextAlign.center,
                              style: TextStyle(
                                color: ProjectColors.main(false),
                                decoration: TextDecoration.none,
                                fontSize: 18,
                                fontWeight: FontWeight.normal,
                              ),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
