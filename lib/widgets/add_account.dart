import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:email_validator/email_validator.dart';

import 'package:mail_app/services/inbox_service.dart';
import 'package:mail_app/services/overlay_builder.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/widgets/custom_button.dart';
import 'package:mail_app/widgets/custom_form_field.dart';

class AddAccount extends StatefulWidget {
  final OverlayBuilder overlayBuilder;
  final InboxService inboxService;

  const AddAccount({
    super.key,
    required this.overlayBuilder,
    required this.inboxService,
  });

  @override
  AddAccountState createState() => AddAccountState();
}

class AddAccountState extends State<AddAccount> {
  late OverlayBuilder _overlayBuilder;
  late InboxService _inboxService;

  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  bool loading = false;

  String? _username;
  String? _password;
  String? _address;
  int? _port;

  @override
  void initState() {
    super.initState();

    _overlayBuilder = widget.overlayBuilder;
    _inboxService = widget.inboxService;
  }

  void _cancel() {
    _overlayBuilder.removeOverlay();
  }

  Future<void> _confirm() async {
    loading = true;

    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();

      if (_username == null ||
          _password == null ||
          _address == null ||
          _port == null) {
        return;
      }

      final connection = await _inboxService.newSession(
        _username!,
        _password!,
        _address!,
        _port!,
      );

      if (connection != -1) {
        print('success');

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
                children: <Widget>[
                  CustomFormField(
                    onSaved: (val) => _username = val,
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
                          onSaved: (val) => _address = val,
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
                              _port = (val != null) ? int.parse(val) : 0,
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
                          onSaved: (val) => _address = val,
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
                              _port = (val != null) ? int.parse(val) : 0,
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
