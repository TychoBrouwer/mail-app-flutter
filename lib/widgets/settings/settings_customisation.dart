import 'package:flutter/material.dart';

class SettingsCustomisation extends StatefulWidget {
  const SettingsCustomisation({super.key});

  @override
  SettingsCustomisationState createState() => SettingsCustomisationState();
}

class SettingsCustomisationState extends State<SettingsCustomisation> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Form(
        key: _formKey,
        child: Padding(
          padding: const EdgeInsets.only(top: 20),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                ],
              ),
              const SizedBox(width: 80),
              Column(
                children: [
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
