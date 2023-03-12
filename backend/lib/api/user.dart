import 'package:shelf/shelf.dart';
import 'package:shelf_router/shelf_router.dart';

class User {
  Router get router {
    final router = Router();

    router.get('/<userID>', (Request request, String userID) {
      return Response.ok('Hello User $userID');
    });

    router.all('/<ignored|.*>', (Request request) => Response.notFound('null'));
    return router;
  }
}
