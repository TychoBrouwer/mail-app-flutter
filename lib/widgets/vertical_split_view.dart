import 'package:flutter/material.dart';

class VerticalSplitView extends StatefulWidget {
  final Widget left;
  final Widget middle;
  final Widget right;
  final double ratio1;
  final double ratio2;
  final double minRatio1;
  final double maxRatio1;
  final double minRatio2;
  final double maxRatio2;

  const VerticalSplitView(
      {super.key,
      required this.left,
      required this.middle,
      required this.right,
      this.ratio1 = 0.15,
      this.ratio2 = 0.15,
      this.minRatio1 = 0.1,
      this.maxRatio1 = 0.2,
      this.minRatio2 = 0.1,
      this.maxRatio2 = 0.2})
      : assert(ratio1 >= 0),
        assert(ratio2 >= 0),
        assert(ratio1 + ratio2 <= 1),
        assert(minRatio1 >= 0),
        assert(maxRatio1 >= 0),
        assert(minRatio2 >= 0),
        assert(maxRatio2 >= 0),
        assert(minRatio1 + minRatio2 <= 1),
        assert(maxRatio1 + maxRatio2 <= 1);

  @override
  VerticalSplitViewStateState createState() => VerticalSplitViewStateState();
}

class VerticalSplitViewStateState extends State<VerticalSplitView> {
  final _dividerWidth = 15.0;

  late double _ratio1;
  late double _ratio2;
  late double _minRatio1;
  late double _maxRatio1;
  late double _minRatio2;
  late double _maxRatio2;

  late double _maxWidth;

  get _width1 => _ratio1 * _maxWidth;
  get _width2 => _ratio2 * _maxWidth;
  get _width3 => _maxWidth - _width1 - _width2;

  @override
  void initState() {
    super.initState();

    _ratio1 = widget.ratio1;
    _ratio2 = widget.ratio2;
    _minRatio1 = widget.minRatio1;
    _maxRatio1 = widget.maxRatio1;
    _minRatio2 = widget.minRatio2;
    _maxRatio2 = widget.maxRatio2;
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, BoxConstraints constraints) {
      assert(_ratio1 + _ratio2 <= 1);
      assert(_ratio1 >= 0);
      assert(_ratio2 >= 0);

      _maxWidth = constraints.maxWidth;

      return SizedBox(
        width: constraints.maxWidth,
        child: Stack(
          alignment: AlignmentDirectional.center,
          children: <Widget>[
            Positioned(
              left: 0,
              top: 0,
              bottom: 0,
              child: SizedBox(
                width: _width1,
                child: widget.left,
              ),
            ),
            Positioned(
              left: _width1,
              top: 0,
              bottom: 0,
              child: SizedBox(
                width: _width2,
                child: widget.middle,
              ),
            ),
            Positioned(
              left: _width1 + _width2,
              top: 0,
              bottom: 0,
              child: SizedBox(
                width: _width3,
                child: widget.right,
              ),
            ),
            Positioned(
              left: _width1 - _dividerWidth / 2,
              top: 0,
              child: GestureDetector(
                behavior: HitTestBehavior.translucent,
                child: MouseRegion(
                  cursor: SystemMouseCursors.resizeColumn,
                  child: SizedBox(
                    width: _dividerWidth,
                    height: constraints.maxHeight,
                  ),
                ),
                onPanUpdate: (DragUpdateDetails details) {
                  final newRatio = _ratio1 + details.delta.dx / _maxWidth;
                  final currentRatio = details.globalPosition.dx / _maxWidth;

                  if (newRatio <= _maxRatio1 &&
                      newRatio >= _minRatio1 &&
                      _maxRatio1 >= currentRatio &&
                      _minRatio1 <= currentRatio) {
                    setState(() {
                      _ratio1 = newRatio;
                    });
                  }
                },
              ),
            ),
            Positioned(
              left: _width1 + _width2 - _dividerWidth / 2,
              top: 0,
              child: GestureDetector(
                behavior: HitTestBehavior.translucent,
                child: MouseRegion(
                  cursor: SystemMouseCursors.resizeColumn,
                  child: SizedBox(
                    width: _dividerWidth,
                    height: constraints.maxHeight,
                  ),
                ),
                onPanUpdate: (DragUpdateDetails details) {
                  final newRatio = _ratio2 + details.delta.dx / _maxWidth;
                  final currentRatio = details.globalPosition.dx / _maxWidth;

                  if (newRatio <= _maxRatio2 &&
                      newRatio >= _minRatio2 &&
                      _ratio1 + _maxRatio2 >= currentRatio &&
                      _ratio1 + _minRatio2 <= currentRatio) {
                    setState(() {
                      _ratio2 = newRatio;
                    });
                  }
                },
              ),
            ),
          ],
        ),
      );
    });
  }
}
