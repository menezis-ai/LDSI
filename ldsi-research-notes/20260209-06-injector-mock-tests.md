# Tests Mock pour l'Injector (wiremock)

**Date** : 2026-02-09
**Tags** : #injector #testing #wiremock
**Status** : Implemente, 11 tests verts

## Contexte

Couverture `injector.rs` a ~25%. Seuls 4 tests synchrones (creation de config, pas d'appels reseau).

## Changement

Ajout de `wiremock = "0.6"` en dev-dependency. 11 nouveaux tests async :

### Happy Path (4 tests)
- `test_ollama_happy_path` : POST /api/generate, 200 OK, parse `response`
- `test_openai_happy_path` : POST /v1/chat/completions, 200 OK, parse `choices[0].message.content`
- `test_anthropic_happy_path` : POST /v1/messages, 200 OK, parse `content[0].text`
- `test_openrouter_happy_path` : POST /v1/chat/completions, 200 OK (meme format OpenAI)

### Erreurs HTTP (2 tests)
- `test_api_error_429_rate_limit` : verifie que 429 -> `InjectorError::ApiError` avec "429"
- `test_api_error_500_server` : verifie que 500 -> `InjectorError::ApiError` avec "500"

### Erreurs de Parsing (2 tests)
- `test_malformed_json_response` : JSON invalide -> `InjectorError::ParseError`
- `test_openai_empty_choices` : `choices: []` -> `ParseError("No response")`

### Validation Pre-Requete (2 tests)
- `test_anthropic_missing_api_key` : pas de key -> `ApiError("API key")` SANS requete reseau
- `test_openrouter_missing_api_key` : idem pour OpenRouter

### Integration (1 test)
- `test_inject_ab_double_call` : verifie que `inject_ab()` fait bien 2 appels (`.expect(2)`)

## Non Couvert

- Retry/backoff sur 429 (pas implemente dans l'injector)
- Timeout (difficile a mocker proprement avec wiremock)
- Streaming responses (injector utilise `stream: false`)

## Liens

- [[20260209-07-benchmark-live]] (les mocks ne remplacent pas un vrai test live)
