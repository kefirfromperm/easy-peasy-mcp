//! The MCP adapter: `initialize`, `tools/list` and `tools/call` over rmcp (TOOLS.md).

use std::sync::Arc;

use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
    ListToolsResult, PaginatedRequestParams, ProgressNotificationParam, ProgressToken,
    ServerCapabilities, ServerConfig, Tool, ToolAnnotations, ToolsCapability,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler};
use serde_json::Value;

use crate::description;
use crate::engine::Engine;
use crate::search::Progress;

/// The MCP server: a thin layer over [`Engine`].
#[derive(Debug, Clone)]
pub struct Server {
    engine: Arc<Engine>,
}

impl Server {
    /// A server for the session of `engine`.
    #[must_use]
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(engine),
        }
    }

    fn tools() -> Vec<Tool> {
        let Value::Object(schema) = description::find_schema() else {
            unreachable!("the schema is a JSON object")
        };
        let Value::Object(get_config_schema) = description::get_config_schema() else {
            unreachable!("the schema is a JSON object")
        };
        let Value::Object(update_config_schema) = description::update_config_schema() else {
            unreachable!("the schema is a JSON object")
        };
        let Value::Object(delete_schema) = description::delete_schema() else {
            unreachable!("the schema is a JSON object")
        };
        vec![
            Tool::new("find", description::FIND_DESCRIPTION, schema)
                .annotate(ToolAnnotations::new().read_only(true).open_world(false)),
            Tool::new(
                "get_config",
                description::GET_CONFIG_DESCRIPTION,
                get_config_schema,
            )
            .annotate(ToolAnnotations::new().read_only(true).open_world(false)),
            Tool::new(
                "update_config",
                description::UPDATE_CONFIG_DESCRIPTION,
                update_config_schema,
            )
            .annotate(
                ToolAnnotations::new()
                    .read_only(false)
                    .destructive(false)
                    .idempotent(true)
                    .open_world(false),
            ),
            Tool::new("delete", description::DELETE_DESCRIPTION, delete_schema).annotate(
                ToolAnnotations::new()
                    .read_only(false)
                    .destructive(true)
                    .idempotent(true)
                    .open_world(false),
            ),
        ]
    }
}

/// Sends `notifications/progress` about once a second, the first after one second (TOOLS.md#progress): the scanned
/// files and `scanned N files, M matched, T.Ts`.
async fn notify_progress(
    peer: rmcp::Peer<RoleServer>,
    token: ProgressToken,
    progress: Arc<Progress>,
    started: std::time::Instant,
) {
    let second = std::time::Duration::from_secs(1);
    let mut ticks = tokio::time::interval_at(tokio::time::Instant::now() + second, second);
    loop {
        ticks.tick().await;
        let scanned = progress.scanned();
        let message = format!(
            "scanned {scanned} files, {} matched, {:.1}s",
            progress.matched(),
            started.elapsed().as_secs_f64()
        );
        #[allow(clippy::cast_precision_loss)] // counts of files far below 2^52
        let param =
            ProgressNotificationParam::new(token.clone(), scanned as f64).with_message(message);
        if peer.notify_progress(param).await.is_err() {
            return;
        }
    }
}

impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        let mut tools = ToolsCapability::default();
        tools.list_changed = Some(false);
        let mut capabilities = ServerCapabilities::default();
        capabilities.tools = Some(tools);
        ServerConfig::new(capabilities)
            .with_server_info(Implementation::new(
                "easy-peasy-mcp",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(description::INSTRUCTIONS)
    }

    #[allow(clippy::unused_async_trait_impl)] // rmcp declares the method async, the list is static
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let listed = self.engine.tools();
        Ok(ListToolsResult::with_all_items(
            Self::tools()
                .into_iter()
                .filter(|t| listed.contains(&t.name.as_ref()))
                .collect(),
        ))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let name = request.name.to_string();
        if !Engine::TOOLS.contains(&name.as_str()) {
            return Err(ErrorData::invalid_params(
                format!("unknown tool '{name}'"),
                None,
            ));
        }
        let arguments = request.arguments.map_or(Value::Null, Value::Object);
        tracing::debug!("tools/call {name} {arguments}");
        let started = std::time::Instant::now();
        let engine = Arc::clone(&self.engine);
        let tool = name.clone();
        let progress = Arc::new(Progress::default());
        let ticker = context.meta.get_progress_token().map(|token| {
            tokio::spawn(notify_progress(
                context.peer.clone(),
                token,
                Arc::clone(&progress),
                started,
            ))
        });
        let cancelled = {
            let progress = Arc::clone(&progress);
            let token = context.ct.clone();
            tokio::spawn(async move {
                token.cancelled().await;
                progress.cancel();
            })
        };
        let counted = Arc::clone(&progress);
        let output = tokio::task::spawn_blocking(move || {
            engine.call_with(&tool, &arguments, Some(&counted))
        })
        .await;
        if let Some(ticker) = ticker {
            ticker.abort();
        }
        cancelled.abort();
        let output = output
            .map_err(|e| ErrorData::internal_error(format!("the search failed: {e}"), None))?
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        tracing::debug!(
            "tools/call {name} done in {} ms, error: {}",
            started.elapsed().as_millis(),
            output.is_error
        );
        let blocks = vec![ContentBlock::text(output.text)];
        let result = if output.is_error {
            CallToolResult::error(blocks)
        } else {
            CallToolResult::success(blocks)
        };
        Ok(result.into())
    }
}
