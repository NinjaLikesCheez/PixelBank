import 'dart:async';
import 'dart:io';

import 'package:args/args.dart';
import 'package:args/command_runner.dart';
import 'dart:convert';

class AccountCommand extends Command {
  @override
  String get description => "Commands that operate on user accounts";

  @override
  String get name => "account";

  @override
  bool get takesArguments => true;

  @override
  FutureOr? run() async {
    print("account command: ${argResults!.arguments}");
    var request = await HttpClient().getUrl(
        Uri.parse('http://localhost:8080/user/${argResults!.arguments.first}'));
    var response = await request.close();
    var list = await response.transform(Utf8Decoder()).toList();
    print(list);

    // .then((request) => request.close())
    // .then((response) => response.transform(Utf8Decoder()).listen(print));
  }
}

class NewAccountCommand extends Command {
  @override
  String get description => "Commands that operate on user accounts";

  @override
  String get name => "new";

  @override
  bool get takesArguments => true;

  @override
  FutureOr? run() {
    // argResults!.arguments to get arguments
    print("New Account Command: ${argResults!.arguments}");
    return null;
  }
}

// abstract class Command {
//   String get name;
//   ArgParser get parser;
// }

// class AccountCommand implements Command {
//   @override
//   String get name => "account";

//   @override
//   ArgParser get parser => ArgParser()
//     ..addOption('create')
//     ..addOption('details');
// }
