import 'package:flutter/material.dart';

class OverlayBuilder {
  late BuildContext _context;
  OverlayEntry? _overlayEntry;

  OverlayBuilder(BuildContext context) {
    _context = context;
  }

  void removeOverlay() {
    _overlayEntry?.remove();
    _overlayEntry = null;
  }

  void insertOverlay(Widget overlay) {
    _overlayEntry = OverlayEntry(
      builder: (context) => overlay,
    );

    Overlay.of(_context)?.insert(_overlayEntry!);
  }
}
