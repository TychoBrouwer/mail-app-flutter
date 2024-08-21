import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart' show SvgPicture;

import 'package:mail_app/services/inbox_service.dart';
import 'package:mail_app/screens/home.dart';
import 'package:mail_app/types/project_colors.dart';
import 'package:mail_app/types/project_sizes.dart';

class SplashPage extends StatefulWidget {
  const SplashPage({super.key});

  @override
  SplashPageState createState() => SplashPageState();
}

class SplashPageState extends State<SplashPage>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller =
      AnimationController(vsync: this, duration: const Duration(seconds: 1))
        ..repeat();

  String _status = '';

  @override
  void initState() {
    super.initState();

    _loadHomePage();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _loadHomePage() async {
    setState(() => _status = 'Loading inboxes');
    await _loadInboxService();

    if (!mounted) return;
    Navigator.pushReplacement(
      context,
      MaterialPageRoute(
        builder: (context) => const HomePage(),
      ),
    );
  }

  Future<void> _loadInboxService() async {
    final inboxService = InboxService();

    final sessions = await inboxService.getSessions();

    if (sessions == null) {
      final inboxService = await _loadInboxService();

      return inboxService;
    }

    if (sessions.isNotEmpty) {
      inboxService.setActiveSessionId(sessions[0].sessionId);
      inboxService.updateMailboxes();
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          color: ProjectColors.background(true),
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
                    fontSize: ProjectSizes.fontSizeExtraLarge,
                    color: ProjectColors.text(true),
                  ),
                ),
              ),
              AnimatedBuilder(
                animation: _controller,
                builder: (BuildContext context, Widget? child) {
                  return Transform.rotate(
                    angle: _controller.value * 2 * 3.14,
                    child: child!,
                  );
                },
                child: SvgPicture.asset(
                  'assets/icons/arrows-rotate.svg',
                  color: ProjectColors.text(true),
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
