import 'package:flutter/material.dart';
// import 'dart:io';
// import 'package:enough_mail/enough_mail.dart';

import 'screens/home.dart';

void main() {
  runApp(const MailApp());
}

class MailApp extends StatelessWidget {
  const MailApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Mail App',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(),
    );
  }
}
