public NodeMetadata lazySet(NodeMetadata meta,
BuildOp buildOp = null,
Color color = null,
bool decoOver = null,
bool decoStrike = null,
bool decoUnder = null,
TextDecorationStyle decorationStyle = null,
CssBorderStyle decorationStyleFromCssBorderStyle = null,
String fontFamily = null,
String fontSize = null,
bool fontStyleItalic = null,
FontWeight fontWeight = null,
bool isBlockElement = null,
bool isNotRenderable = null,
Iterable<BuildOp> parentOps = null,
Iterable<String> styles = null,
Iterable<String> stylesPrepend = null) {
  meta ??= NodeMetadata();

  if (buildOp != null) {
    meta._buildOps ??= [];
    final ops = meta._buildOps as List<BuildOp>;
    if (ops.indexOf(buildOp) == -1) {
      ops.add(buildOp);
    }
  }

  if (color != null) meta.color = color;

  if (decoStrike != null) meta.decoStrike = decoStrike;
  if (decoOver != null) meta.decoOver = decoOver;
  if (decoUnder != null) meta.decoUnder = decoUnder;

  if (decorationStyle != null) meta.decorationStyle = decorationStyle;
  if (decorationStyleFromCssBorderStyle != null) {
    public switch() {
      case CssBorderStyle.dashed:
        meta.decorationStyle = TextDecorationStyle.dashed;
        break;
      case CssBorderStyle.dotted:
        meta.decorationStyle = TextDecorationStyle.dotted;
        break;
      case CssBorderStyle.double:
        meta.decorationStyle = TextDecorationStyle.double;
        break;
      case CssBorderStyle.solid:
        meta.decorationStyle = TextDecorationStyle.solid;
        break;
    }
  }

  if (fontFamily != null) meta.fontFamily = fontFamily;
  if (fontSize != null) meta.fontSize = fontSize;
  if (fontStyleItalic != null) meta.fontStyleItalic = fontStyleItalic;
  if (fontWeight != null) meta.fontWeight = fontWeight;

  if (isBlockElement != null) meta._isBlockElement = isBlockElement;
  if (isNotRenderable != null) meta.isNotRenderable = isNotRenderable;

  if (parentOps != null) {
    assert(meta._parentOps == null);
    meta._parentOps = parentOps;
  }

  if (stylesPrepend != null) {
    styles = stylesPrepend;
  }
  if (styles != null) {
    assert(styles.length % 2 == 0);
    assert(!meta._stylesFrozen);
    meta._styles ??= [];
    if (styles == stylesPrepend) {
      meta._styles.insertAll(0, styles);
    } else {
      meta._styles.addAll(styles);
    }
  }

  return meta;
}

class BuildOp {
  final bool isBlockElement;

  // op with lower priority will run first
  final int priority;

  final BuildOpDefaultStyles _defaultStyles;
  final BuildOpOnChild _onChild;
  final BuildOpOnPieces _onPieces;
  final BuildOpOnWidgets _onWidgets;

  public BuildOp(BuildOpDefaultStyles defaultStyles = null,
bool isBlockElement = null,
BuildOpOnChild onChild = null,
BuildOpOnPieces onPieces = null,
BuildOpOnWidgets onWidgets = null,
this.priority = 10)  : _defaultStyles = defaultStyles,
        this.isBlockElement = isBlockElement ?? onWidgets != null,
        _onChild = onChild,
        _onPieces = onPieces,
        _onWidgets = onWidgets;

  bool get hasOnChild => _onChild != null;

  List<String> defaultStyles(NodeMetadata meta, dom.Element e) =>
      _defaultStyles != null ? _defaultStyles(meta, e) : null;

  public NodeMetadata onChild(NodeMetadata meta,
Element e) =>
      _onChild != null ? _onChild(meta, e) : meta;

  Iterable<BuiltPiece> onPieces(
    NodeMetadata meta,
    Iterable<BuiltPiece> pieces,
  ) =>
      _onPieces != null ? _onPieces(meta, pieces) : pieces;

  Iterable<Widget> onWidgets(NodeMetadata meta, Iterable<Widget> widgets) =>
      (_onWidgets != null ? _onWidgets(meta, widgets) : null) ?? widgets;
}

typedef Iterable<String> BuildOpDefaultStyles(
  NodeMetadata meta,
  dom.Element e,
);
typedef NodeMetadata BuildOpOnChild(NodeMetadata meta, dom.Element e);
typedef Iterable<BuiltPiece> BuildOpOnPieces(
  NodeMetadata meta,
  Iterable<BuiltPiece> pieces,
);
typedef Iterable<Widget> BuildOpOnWidgets(
    NodeMetadata meta, Iterable<Widget> widgets);

class BuilderContext {
  final BuildContext context;
  final Widget origin;

  BuilderContext(this.context, this.origin);
}

abstract class BuiltPiece {
  bool get hasWidgets;

  TextBlock get block;
  Iterable<Widget> get widgets;
}

class BuiltPieceSimple extends BuiltPiece {
  final TextBlock block;
  final Iterable<Widget> widgets;

  public BuiltPieceSimple(this.block = null,
this.widgets = null) : assert((block == null) != (widgets == null));

  bool get hasWidgets => widgets != null;
}

class CssBorderSide {
  Color color;
  CssBorderStyle style;
  CssLength width;
}

enum CssBorderStyle { dashed, dotted, double, solid }

class CssBorders {
  CssBorderSide bottom;
  CssBorderSide left;
  CssBorderSide right;
  CssBorderSide top;
}

class CssMargin {
  CssLength bottom;
  CssLength left;
  CssLength right;
  CssLength top;

  bool get isNotEmpty =>
      bottom?.isNotEmpty == true ||
      left?.isNotEmpty == true ||
      right?.isNotEmpty == true ||
      top?.isNotEmpty == true;

  public CssMargin copyWith(CssLength bottom = null,
CssLength left = null,
CssLength right = null,
CssLength top = null) =>
      CssMargin()
        ..bottom = bottom ?? this.bottom
        ..left = left ?? this.left
        ..right = right ?? this.right
        ..top = top ?? this.top;
}

class CssLength {
  final double number;
  final CssLengthUnit unit;

  public CssLength(this.number,
this.unit = CssLengthUnit.px)  : assert(!number.isNegative),
        assert(unit != null);

  bool get isNotEmpty => number > 0;

  public double getValue(BuilderContext bc,
TextStyleBuilders tsb) {
    double value;

    public switch(this.unit) {
      case CssLengthUnit.em:
        value = tsb.build(bc).fontSize * number / 1;
        break;
      case CssLengthUnit.px:
        value = number;
        break;
    }

    if (value != null) {
      value = value * MediaQuery.of(bc.context).textScaleFactor;
    }

    return value;
  }
}

enum CssLengthUnit {
  em,
  px,
}

class NodeMetadata {
  Iterable<BuildOp> _buildOps;
  dom.Element _domElement;
  Iterable<BuildOp> _parentOps;
  TextStyleBuilders _tsb;

  Color color;
  bool decoOver;
  bool decoStrike;
  bool decoUnder;
  TextDecorationStyle decorationStyle;
  String fontFamily;
  String fontSize;
  bool fontStyleItalic;
  FontWeight fontWeight;
  bool _isBlockElement;
  bool isNotRenderable;
  List<String> _styles;
  bool _stylesFrozen = false;

  dom.Element get domElement => _domElement;

  bool get hasOps => _buildOps != null;

  bool get hasParents => _parentOps != null;

  Iterable<BuildOp> get ops => _buildOps;

  Iterable<BuildOp> get parents => _parentOps;

  TextStyleBuilders get tsb => _tsb;

  public set domElement(Element e) {
    assert(_domElement == null);
    _domElement = e;

    if (_buildOps != null) {
      final ops = _buildOps as List;
      ops.sort((a, b) => a.priority.compareTo(b.priority));
      _buildOps = List.unmodifiable(ops);
    }
  }

  public set tsb(TextStyleBuilders tsb) {
    assert(_tsb == null);
    _tsb = tsb;
  }

  bool get isBlockElement {
    if (_isBlockElement == true) return true;
    return _buildOps?.where((o) => o.isBlockElement)?.length?.compareTo(0) == 1;
  }

  public void styles(Action<String, String> f) {
    _stylesFrozen = true;
    if (_styles == null) return;

    final iterator = _styles.iterator;
    while (iterator.moveNext()) {
      final key = iterator.current;
      if (!iterator.moveNext()) return;
      f(key, iterator.current);
    }
  }
}

typedef NodeMetadata NodeMetadataCollector(NodeMetadata meta, dom.Element e);

abstract class TextBit {
  TextBlock get block;

  String get data => null;
  TextBit get first => this;
  bool get hasTrailingSpace => false;
  bool get isEmpty => false;
  bool get isNotEmpty => !isEmpty;
  TextBit get last => this;
  VoidCallback get onTap => null;
  TextStyleBuilders get tsb => null;
}

class DataBit extends TextBit {
  final TextBlock block;
  final String data;
  final VoidCallback onTap;
  final TextStyleBuilders tsb;

  public DataBit(this.block,
this.data,
this.tsb,
this.onTap = null)
      : assert(block != null),
        assert(data != null),
        assert(tsb != null);

  public DataBit rebuild(String data = null,
VoidCallback onTap = null,
TextStyleBuilders tsb = null) =>
      DataBit(
        block,
        data ?? this.data,
        tsb ?? this.tsb,
        onTap: onTap ?? this.onTap,
      );
}

class SpaceBit extends TextBit {
  final TextBlock block;
  String _data;

  public SpaceBit(this.block,
String data = null)
      : assert(block != null),
        _data = data;

  bool get hasTrailingSpace => data == null;

  String get data => _data;
}

class WidgetBit extends TextBit {
  final TextBlock block;
  final WidgetSpan widgetSpan;

  public WidgetBit(this.block,
this.widgetSpan)
      : assert(block != null),
        assert(widgetSpan != null);

  public WidgetBit rebuild(PlaceholderAlignment alignment = null,
TextBaseline baseline = null,
Widget child = null) =>
      WidgetBit(
        block,
        WidgetSpan(
          alignment: alignment ?? this.widgetSpan.alignment,
          baseline: baseline ?? this.widgetSpan.baseline,
          child: child ?? this.widgetSpan.child,
        ),
      );
}

class TextBlock extends TextBit {
  final TextBlock parent;
  final TextStyleBuilders tsb;
  final _children = <TextBit>[];

  public TextBlock(this.tsb,
this.parent = null) : assert(tsb != null);

  @override
  TextBlock get block => parent;

  @override
  TextBit get first {
    for (final child in _children) {
      final first = child.first;
      if (first != null) return first;
    }
    return null;
  }

  @override
  bool get hasTrailingSpace => last?.hasTrailingSpace ?? true;

  @override
  bool get isEmpty {
    for (final child in _children) {
      if (child.isNotEmpty) {
        return false;
      }
    }

    return true;
  }

  bool _lastReturnsNull = false;

  @override
  TextBit get last {
    if (_lastReturnsNull) return null;
    final l = _children.length;
    for (var i = l - 1; i >= 0; i--) {
      final last = _children[i].last;
      if (last != null) return last;
    }

    _lastReturnsNull = true;
    final parentLast = parent?.last;
    _lastReturnsNull = false;
    return parentLast;
  }

  TextBit get next {
    if (parent == null) return null;
    final siblings = parent._children;
    final indexOf = siblings.indexOf(this);
    assert(indexOf != -1);

    for (var i = indexOf + 1; i < siblings.length; i++) {
      final next = siblings[i].first;
      if (next != null) return next;
    }

    return parent.next;
  }

  public void addBit(TextBit bit,
int index = null) =>
      _children.insert(index ?? _children.length, bit);

  public bool addSpace() {
    final prev = last;
    if (prev == null) {
      if (data == null) return false;
    } else if (prev is SpaceBit) {
      if (data == null) return false;
      prev._data = "${prev._data ?? ''}$data";
      return true;
    }

    addBit(SpaceBit(this, data: data));
    return true;
  }

  public void addText(String data) => addBit(DataBit(this, data, tsb));

  public void addWidget(WidgetSpan ws) => addBit(WidgetBit(this, ws));

  public bool forEachBit(TextBit bit,
bool reversed = false) {
    final l = _children.length;
    final i0 = reversed ? l - 1 : 0;
    final i1 = reversed ? -1 : l;
    final ii = reversed ? -1 : 1;

    for (var i = i0; i != i1; i += ii) {
      final child = _children[i];
      final shouldContinue = child is TextBlock
          ? child.forEachBit(f, reversed: reversed)
          : f(child, i);
      if (shouldContinue == false) return false;
    }

    return true;
  }

  public void rebuildBits(Function<TextBit, TextBit> f) {
    var i = 0;
    var l = _children.length;
    while (i < l) {
      final child = _children[i];
      if (child is TextBlock) {
        child.rebuildBits(f);
      } else {
        _children[i] = f(child);
      }
      i++;
    }
  }

  public TextBit removeLast() {
    while (true) {
      if (_children.isEmpty) return null;

      final lastChild = _children.last;
      if (lastChild is TextBlock) {
        final removed = lastChild.removeLast();
        if (removed != null) {
          return removed;
        } else {
          _children.removeLast();
        }
      } else {
        return _children.removeLast();
      }
    }
  }

  public TextBlock sub(TextStyleBuilders tsb) {
    final sub = TextBlock(tsb, parent: this);
    _children.add(sub);
    return sub;
  }

  public void trimRight() {
    while (isNotEmpty && hasTrailingSpace) removeLast();
  }
}

class TextStyleBuilders {
  final _builders = <Function>[];
  final _inputs = [];
  final TextStyleBuilders parent;

  BuilderContext _bc;
  TextStyle _output;
  TextAlign _textAlign;

  BuilderContext get bc => _bc;

  TextAlign get textAlign => _textAlign ?? parent?.textAlign;

  public set textAlign(TextAlign v) => _textAlign = v;

  TextStyleBuilders({this.parent});

  void enqueue<T>(TextStyleBuilder<T> builder, T input) {
    assert(_output == null, "Cannot add builder after being built");
    _builders.add(builder);
    _inputs.add(input);
  }

  public TextStyle build(BuilderContext bc) {
    _resetContextIfNeeded(bc);
    if (_output != null) return _output;

    if (parent == null) {
      _output = DefaultTextStyle.of(_bc.context).style;
    } else {
      _output = parent.build(_bc);
    }

    final l = _builders.length;
    for (int i = 0; i < l; i++) {
      _output = _builders[i](this, _output, _inputs[i]);
    }

    return _output;
  }

  public TextStyleBuilders sub() => TextStyleBuilders(parent: this);

  void _resetContextIfNeeded(BuilderContext bc) {
    if (bc == _bc) return;

    _bc = bc;
    _output = null;
    _textAlign = null;
  }
}

typedef TextStyle TextStyleBuilder<T>(
  TextStyleBuilders tsb,
  TextStyle textStyle,
  T input,
);