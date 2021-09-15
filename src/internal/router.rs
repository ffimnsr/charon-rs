pub(crate) fn router(config_path: &String) -> ServiceResult<Router<Body, ApiError>> {
    let dbpool = get_db_pool()?;
    let config = parse_config(&config_path)?;

    Router::<Body, ApiError>::builder()
        .data(config.clone())
        .data(dbpool.clone())
        .get("/clients", handler_index)
        .post("/clients", handler_index)
        .get("/clients/:id", handler_index)
        .update("/clients/:id", handler_index)
        .delete("/clients/:id", handler_index)
        .patch("/clients/:id", handler_index)
        .get("/health/alive", handler_index)
        .get("/keys/:set", handler_index)
        .put("/keys/:set", handler_index)
        .post("/keys/:set", handler_index)
        .delete("/keys/:set", handler_index)
        .get("/keys/:set/:kid", handler_index)
        .put("/keys/:set/:kid", handler_index)
        .delete("/keys/:set/:kid", handler_index)
        .get("/oauth2/auth/requests/consent", handler_index)
        .put("/oauth2/auth/requests/consent/accept", handler_index)
        .put("/oauth2/auth/requests/consent/reject", handler_index)
        .get("/oauth2/auth/requests/login", handler_index)
        .put("/oauth2/auth/requests/login/accept", handler_index)
        .put("/oauth2/auth/requests/login/reject", handler_index)
        .get("/oauth2/auth/requests/logout", handler_index)
        .put("/oauth2/auth/requests/logout/accept", handler_index)
        .put("/oauth2/auth/requests/logout/reject", handler_index)
        .get("/oauth2/auth/sessions/consent", handler_index)
        .delete("/oauth2/auth/sessions/login", handler_index)
        .post("/oauth2/flush", handler_index)
        .post("/oauth2/introspect", handler_index)
        .delete("/oauth2/tokens", handler_index)
        .get("/version", handler_index)

        .any(handler_not_found)
        .err_handler(error_handler)
        .build()
        .map_err(|e| ServiceError::Router(e.to_string()))
}
