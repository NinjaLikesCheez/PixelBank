import 'package:backend/backend.dart' as backend;

import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart' as shelf_io;

void main() async {
  final router = backend.APIRouter();
  final server = await shelf_io.serve(router.handler, 'localhost', 8080);
  print('Serving at http://${server.address.host}:${server.port}');
}
