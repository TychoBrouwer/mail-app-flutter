import 'package:flutter/material.dart';
import 'package:flutter/services.dart' show FilteringTextInputFormatter;
import 'package:email_validator/email_validator.dart' show EmailValidator;

import '../services/inbox_service.dart';
import '../services/overlay_builder.dart';
import '../types/project_colors.dart';
import '../types/project_sizes.dart';
import 'custom_button.dart';
import 'custom_form_field.dart';

class AddAccount extends StatefulWidget {
  const AddAccount({super.key});

  @override
  AddAccountState createState() => AddAccountState();
}

class AddAccountState extends State<AddAccount> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  bool loading = false;

  String? _username;
  String? _password;
  String? _address;
  int? _port;

  @override
  void initState() {
    super.initState();
  }

  void _cancel() {
    OverlayBuilder().removeOverlay(0);
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

      final connection = await InboxService().newSession(
        _username!,
        _password!,
        _address!,
        _port!,
      );

      if (connection != -1) {
        print('success');

        OverlayBuilder().removeOverlay(0);
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
      child: Center(
        child: Container(
          height: 360,
          width: 550,
          decoration: BoxDecoration(
            border: Border.all(color: ProjectColors.border(true)),
            color: ProjectColors.background(true),
            borderRadius: ProjectSizes.borderRadiusExtraSmall,
            boxShadow: const [
              BoxShadow(
                color: Colors.black,
                blurRadius: 10,
                spreadRadius: 0,
              ),
            ],
          ),
          child: Form(
            key: _formKey,
            child: Padding(
              padding: const EdgeInsets.all(50),
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
                              return 'Enter IMAP Port';
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
                                color: ProjectColors.text(true),
                                decoration: TextDecoration.none,
                                fontSize: ProjectSizes.fontSizeLarge,
                                fontWeight: FontWeight.w500,
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
                                color: ProjectColors.text(true),
                                decoration: TextDecoration.none,
                                fontSize: ProjectSizes.fontSizeLarge,
                                fontWeight: FontWeight.w500,
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
