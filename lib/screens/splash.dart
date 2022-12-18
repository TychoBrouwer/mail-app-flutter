import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:mail_app/screens/home.dart';
import 'package:mail_app/services/inbox_service.dart';
import 'package:mail_app/services/local_settings.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/utils/local_file_store.dart';

class SplashPage extends StatefulWidget {
  const SplashPage({super.key});

  @override
  State<SplashPage> createState() => _SplashPageState();
}

class _SplashPageState extends State<SplashPage> {
  late LocalFileStore _fileStore;
  late LocalSettings _localSettings;

  final InboxService _inboxService = InboxService();
  double _turns = 0;
  String _status = '';

  @override
  void initState() {
    super.initState();

    _loadHomePage();
  }

  void _loadHomePage() async {
    setState(() => _status = 'Loading application settings');
    await _loadSettings();
    setState(() => _status = 'Loading inboxes');
    setState(() => _turns += 100);
    await _loadInboxService();

    if (!mounted) return;
    Navigator.pushReplacement(
      context,
      MaterialPageRoute(
        builder: (context) => HomePage(
          fileStore: _fileStore,
          localSettings: _localSettings,
          inboxService: _inboxService,
        ),
      ),
    );
  }

  Future<void> _loadSettings() async {
    _fileStore = LocalFileStore();
    _localSettings = LocalSettings(_fileStore);

    await _localSettings.loaded();
  }

  Future<void> _loadInboxService() async {
    final accounts = _localSettings.getSettings().accounts();

    for (var account in accounts) {
      _inboxService.newClient(
          account.email,
          account.password,
          account.imapAddress,
          account.imapPort,
          account.smtpAddress,
          account.smtpPort);
    }

    await _inboxService.clientsConnected();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          color: ProjectColors.secondary(true),
        ),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Padding(
                padding: const EdgeInsets.only(bottom: 30),
                child: Text(
                  _status,
                  style: TextStyle(
                    fontSize: 24,
                    color: ProjectColors.main(false),
                  ),
                ),
              ),
              AnimatedRotation(
                alignment: Alignment.center,
                turns: _turns,
                duration: const Duration(seconds: 100),
                child: SvgPicture.asset(
                  'assets/icons/arrows-rotate.svg',
                  color: ProjectColors.main(false),
                  width: 60,
                  height: 60,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
