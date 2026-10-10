//! The MCP adapter: `initialize`, `tools/list` and `tools/call` over rmcp (TOOLS.md).

use std::sync::Arc;

use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
    ListToolsResult, PaginatedRequestParams, ServerCapabilities, ServerConfig, Tool,
    ToolAnnotations, ToolsCapability,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler};
use serde_json::Value;

use crate::description;
use crate::engine::Engine;

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
        ]
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
        Ok(ListToolsResult::with_all_items(Self::tools()))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let name = request.name.to_string();
        if !Engine::TOOLS.contains(&name.as_str()) {
            return Err(ErrorData::invalid_params(
                format!("unknown tool '{name}'"),
                None,
            ));
        }
        let arguments = request.arguments.map_or(Value::Null, Value::Object);
        let engine = Arc::clone(&self.engine);
        let output = tokio::task::spawn_blocking(move || engine.call(&name, &arguments))
            .await
            .map_err(|e| ErrorData::internal_error(format!("the search failed: {e}"), None))?
            .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
        let content = vec![ContentBlock::text(output.text)];
        let result = if output.is_error {
            CallToolResult::error(content)
        } else {
            CallToolResult::success(content)
        };
        Ok(result.into())
    }
}
