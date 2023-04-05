import 'package:shell/command.dart';
import 'package:shell/shell.dart' as shell;
import 'package:shell/ansi_colors.dart';
import 'package:args/args.dart';
import 'package:args/command_runner.dart';
import 'package:args/src/help_command.dart';

import 'dart:io';

/* TODO: 
  * Use https://api.dart.dev/stable/2.19.4/dart-isolate/Isolate/spawnUri.html to make a plugin system?
*/

final _runner = _createArgumentParser();

void main(List<String> arguments) {
  stdout.writeln(
      "👋 You are now using PixelBank. All your moneyez r belong to us 🏴‍☠️");

  _allowNewInput();

  stdin.listen((event) async {
    _handleEvent(event);
  });
}

void _handleEvent(List<int> event) async {
  // first, convert the event bytes to a string
  var line = String.fromCharCodes(event);

  // now, pass it off to the command runner to handle all the hard parts for us
  await _runArguments(line.replaceAll('\n', '').split(' '));

  // accept new input
  _allowNewInput();
}

Future<void> _runArguments(List<String> args) async => _runner.run(args);

List<Command> _baseCommands() => [AccountCommand()];

void _allowNewInput() {
  stdout.write('\n${ANSI.escape}${ANSI.green}${ANSI.bold}>${ANSI.reset} ');
}

CommandRunner _createArgumentParser() {
  var runner = CommandRunner('', 'shell interface for the PixelBank');

  for (var command in _baseCommands()) {
    runner.addCommand(command);
  }

  return runner;
}
