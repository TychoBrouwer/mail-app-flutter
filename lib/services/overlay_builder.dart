import 'package:flutter/material.dart';

class OverlayBuilder {
  late BuildContext _context;
  final Map<int, OverlayEntry?> _overlayEntries = {};

  OverlayBuilder(BuildContext context) {
    _context = context;
  }

  void removeOverlay(idx) {
    _overlayEntries[idx]?.remove();
    _overlayEntries[idx] = null;
  }

  void insertOverlay(Widget overlay, int idx) {
    _overlayEntries[idx] = OverlayEntry(
      builder: (context) => overlay,
    );

    Overlay.of(_context).insert(_overlayEntries[idx]!);
  }
}
