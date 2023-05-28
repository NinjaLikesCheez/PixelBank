// If only Dart supported nested classes :')
class ANSI {
  static final escape = '\x1B[';
  static final reset = '${escape}0m';

  // Colors
  static final green = '${escape}32;1m';

  // Formatting
  static final bold = '${escape}1m';
}
