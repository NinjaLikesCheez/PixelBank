import 'package:shelf/shelf.dart';
import 'package:shelf_router/shelf_router.dart';

import 'api/user.dart';

class APIRouter {
  Handler get handler {
    final router = Router();

    router.get('/ping', (Request request) {
      return Response.ok(null);
    });

    // Returns details about a user
    router.mount('/user/', User().router);

    // Returns details about all users
    router.get('/users', (Request request, String userID) async {
      return Response.ok('TODO: list all users');
    });

    router.post('/register/<username>',
        (Request request, String username) async {
      return Response.ok('TODO: registration of new user');
    });

    // Buy an item
    router.get('/buy/<itemID|[0-9]+>', (Request request, int itemID) async {
      return Response.ok('TODO: Buy item with ID: $itemID');
    });

    // Deposits money into a users account
    router.post('/deposit/<amount|[0-9]+>',
        (Request request, int amount) async {
      return Response.ok('Deposit an amount of: $amount');
    });

    // Transfers money into a users account
    router.post('/transfer/<amount|[0-9]+>',
        (Request request, int amount) async {
      return Response.ok('Transfer amount: $amount');
    });

    // router.get('/admin', (Request request) async {
    //   // TODO: Add a new middleware router for admin requests which are authenticated etc
    // });

    // Catch-all 404
    router.all('/<ignored|.*>', (Request request) {
      return Response.notFound('Page not found');
    });

    return router;
  }
}
