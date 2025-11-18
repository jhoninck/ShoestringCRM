# Flutter Client Guide

This document explains how a Flutter client is expected to talk to the
CRM Platform backend.

The design goals:

- Keep all interaction **GraphQL-first**
- Use the **same schema** and patterns as the backend (Hygraph-style)
- Use **ZITADEL** for authentication (OIDC / OAuth 2.0)
- Make it easy to plug in codegen for typed GraphQL models

---

## 1. Dependencies

Example minimal dependencies (pubspec excerpt):

```yaml
dependencies:
  flutter:
    sdk: flutter

  flutter_appauth: ^6.0.0        # OIDC login with ZITADEL
  flutter_secure_storage: ^9.0.0 # persist tokens securely
  graphql_flutter: ^5.1.2        # GraphQL client
```

You may replace `graphql_flutter` with `ferry` or another client if you
prefer code generation.

---

## 2. Authentication with ZITADEL

1. Configure a **native / SPA client** in ZITADEL for your Flutter app.
2. Collect the following values:

   - `issuer` = `https://<your-zitadel-domain>`
   - `clientId` = `<your-flutter-client-id>`
   - `redirectUrl` = e.g. `com.example.crm://callback`
   - `scopes` = `["openid", "profile", "email", "offline_access", "crm-platform-api"]`

3. Perform login using `flutter_appauth`:

```dart
final appAuth = FlutterAppAuth();
final result = await appAuth.authorizeAndExchangeCode(
  AuthorizationTokenRequest(
    clientId,
    redirectUrl,
    discoveryUrl: '$issuer/.well-known/openid-configuration',
    scopes: scopes,
  ),
);

final accessToken = result?.accessToken;
final idToken = result?.idToken;
```

4. Store tokens securely with `flutter_secure_storage`, and attach
   `Authorization: Bearer <accessToken>` to every GraphQL request.

---

## 3. GraphQL Client Setup

Example with `graphql_flutter`:

```dart
import 'package:graphql_flutter/graphql_flutter.dart';

ValueNotifier<GraphQLClient> createClient(String accessToken) {
  final httpLink = HttpLink('http://localhost:8080/graphql');

  final authLink = AuthLink(
    getToken: () async => 'Bearer $accessToken',
  );

  final link = authLink.concat(httpLink);

  return ValueNotifier(
    GraphQLClient(
      cache: GraphQLCache(),
      link: link,
    ),
  );
}
```

In your Flutter app, wrap the widget tree with `GraphQLProvider`:

```dart
GraphQLProvider(
  client: client,
  child: MyApp(),
)
```

---

## 4. Example Query: List Accounts

The backend exposes a Hygraph-style query:

```graphql
query Accounts($first: Int, $after: String, $where: AccountWhereInput) {
  accounts(first: $first, after: $after, where: $where) {
    pageInfo {
      hasNextPage
      endCursor
    }
    edges {
      cursor
      node {
        id
        name
        industry
      }
    }
  }
}
```

Flutter usage:

```dart
final query = r'''
  query Accounts($first: Int, $after: String, $where: AccountWhereInput) {
    accounts(first: $first, after: $after, where: $where) {
      pageInfo {
        hasNextPage
        endCursor
      }
      edges {
        cursor
        node {
          id
          name
          industry
        }
      }
    }
  }
''';

Query(
  options: QueryOptions(
    document: gql(query),
    variables: {
      'first': 20,
      'after': '0',
      'where': {
        'name': {'contains': 'Acme'},
      },
    },
  ),
  builder: (result, {fetchMore, refetch}) {
    if (result.isLoading) return const CircularProgressIndicator();
    if (result.hasException) return Text(result.exception.toString());

    final edges = result.data?['accounts']['edges'] as List<dynamic>;
    // Map edges to your own Account model.
    return ListView.builder(
      itemCount: edges.length,
      itemBuilder: (context, index) {
        final node = edges[index]['node'];
        return ListTile(
          title: Text(node['name']),
          subtitle: Text(node['industry'] ?? ''),
        );
      },
    );
  },
);
```

---

## 5. Example Mutation: Create Account (Future)

When the backend exposes `createAccount`, it will follow the pattern:

```graphql
mutation CreateAccount($data: AccountCreateInput!) {
  createAccount(data: $data) {
    id
    name
  }
}
```

From Flutter, you send:

```dart
final mutation = r'''
  mutation CreateAccount($data: AccountCreateInput!) {
    createAccount(data: $data) {
      id
      name
    }
  }
''';

final result = await client.mutate(
  MutationOptions(
    document: gql(mutation),
    variables: {
      'data': {
        'name': 'Acme Corp',
        'industry': 'Manufacturing',
      },
    },
  ),
);
```

---

## 6. Error Handling & Refresh Tokens

- Use `offline_access` scope in ZITADEL so the client gets
  `refresh_token` in addition to `access_token`.
- Use `flutter_appauth`'s `TokenRequest` to refresh tokens automatically
  when calls fail with 401.
- Centralize token refresh logic in a small auth service so widgets
  never deal with low-level auth.

---

## 7. Recommended Next Steps

- Integrate a GraphQL code generator (e.g. `artemis`, `gql_build_runner`)
  so Dart models stay in sync with the schema.
- Add dedicated pages for:
  - Accounts list / detail
  - Contacts per account
  - Tickets per contact
  - Social timeline per account/contact
- Use the same connection pattern (`pageInfo` + `edges`) everywhere.
