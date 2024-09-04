import 'package:flutter/material.dart';

class SettingsAccount extends StatefulWidget {
  const SettingsAccount({super.key});

  @override
  SettingsAccountState createState() => SettingsAccountState();
}

class SettingsAccountState extends State<SettingsAccount> {
  @override
  Widget build(BuildContext context) {
    return Center(
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        mainAxisSize: MainAxisSize.min,
        children: [
          Container(),
          Expanded(child: Container()),
        ],
      ),
    );
  }
}
