#![allow(dead_code)]

// --8<-- [start:basic_tool]
#[strands::tool(description = "Get weather forecast for a city.")]
async fn weather_forecast(
    city: String,
    days: Option<u32>,
) -> Result<String, strands::ToolError> {
    Ok(format!(
        "Weather forecast for {city} for the next {} days",
        days.unwrap_or(3)
    ))
}
// --8<-- [end:basic_tool]

// --8<-- [start:override_tool]
#[strands::tool(
    name = "get_weather",
    description = "Retrieves weather forecast for a specified location"
)]
async fn forecast(city: String) -> String {
    format!("Sunny in {city}")
}
// --8<-- [end:override_tool]

fn custom_schema() {
    // --8<-- [start:custom_schema]
    let area = strands::tools::FunctionTool::builder(
        "calculate_area",
        "Calculate a shape's area",
    )
        .input_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "shape": {"type": "string", "enum": ["circle", "rectangle"]},
                "radius": {"type": "number"},
                "width": {"type": "number"},
                "height": {"type": "number"}
            },
            "required": ["shape"]
        }))
        .callback(|input, _context| async move {
            let result = match input["shape"].as_str() {
                Some("circle") => {
                    input["radius"]
                        .as_f64()
                        .unwrap_or_default()
                        .powi(2)
                        * std::f64::consts::PI
                }
                Some("rectangle") => {
                    input["width"]
                        .as_f64()
                        .unwrap_or_default()
                        * input["height"]
                            .as_f64()
                            .unwrap_or_default()
                }
                _ => 0.0,
            };
            Ok(serde_json::json!(result))
        });
    // --8<-- [end:custom_schema]
    let _ = area;
}

fn load_tool() -> strands::Result<strands::Agent> {
    // --8<-- [start:load_tool]
    #[strands::tool(description = "Get a weather forecast for a city.")]
    async fn weather_forecast(city: String) -> Result<String, strands::ToolError> {
        Ok(format!("Sunny in {city}"))
    }

    strands::Agent::builder().tools([WeatherForecast]).build()
    // --8<-- [end:load_tool]
}

// --8<-- [start:custom_return]
#[strands::tool(description = "Fetch structured data from a source.")]
async fn fetch_data(source_id: String) -> Vec<strands::ToolResultContent> {
    vec![
        strands::ToolResultContent::text(format!("Loaded {source_id}")),
        strands::ToolResultContent::json(serde_json::json!({"records": 3})),
    ]
}
// --8<-- [end:custom_return]

async fn async_invocation() -> strands::Result<()> {
    // --8<-- [start:async_invocation]
    #[strands::tool(description = "Get a weather forecast for a city.")]
    async fn weather_forecast(city: String) -> Result<String, strands::ToolError> {
        Ok(format!("Sunny in {city}"))
    }

    let mut agent = strands::Agent::builder().tools([WeatherForecast]).build()?;
    agent.invoke("Forecast Seattle for five days.").await?;
    // --8<-- [end:async_invocation]
    Ok(())
}

// --8<-- [start:tool_context]
#[strands::tool(description = "Return the invoking agent's name.")]
async fn get_agent_name(#[tool(context)] context: &strands::ToolContext) -> String {
    context.agent().name().to_owned()
}
// --8<-- [end:tool_context]

// --8<-- [start:custom_context_name]
#[strands::tool(description = "Return this tool use's identifier.")]
async fn get_tool_use_id(#[tool(context)] call: &strands::ToolContext) -> String {
    call.tool_use().tool_use_id.clone()
}
// --8<-- [end:custom_context_name]

// --8<-- [start:tool_state]
#[strands::tool(description = "Return the current tenant.")]
async fn get_tenant(
    #[tool(context)] context: &strands::ToolContext,
) -> Result<String, strands::ToolError> {
    context
        .agent()
        .app_state()
        .get::<String>("tenant")
        .map(|tenant| tenant.unwrap_or_else(|| "default".to_owned()))
        .map_err(strands::ToolError::new)
}
// --8<-- [end:tool_state]

fn streaming_tool() {
    // --8<-- [start:tool_streaming]
    let process_dataset = strands::tools::FunctionTool::builder(
        "process_dataset",
        "Process records with progress updates.",
    )
        .input_schema(serde_json::json!({
            "type": "object",
            "properties": {
                "records": {
                    "type": "integer",
                    "minimum": 0
                }
            },
            "required": ["records"]
        }))
        .stream_callback(|input, _context| {
            let records =
                input["records"].as_u64().unwrap_or_default();
            futures::stream::iter([
                Ok(strands::tools::FunctionToolItem::Update(
                    serde_json::json!({
                        "processed": records / 2,
                        "total": records
                    }),
                )),
                Ok(strands::tools::FunctionToolItem::Return(
                    serde_json::json!({"processed": records}),
                )),
            ])
        });
    // --8<-- [end:tool_streaming]
    let _ = process_dataset;
}

// --8<-- [start:class_tools]
struct DatabaseTools;

#[strands::tools]
impl DatabaseTools {
    #[strands::tool(description = "Run a query against the database.")]
    async fn query_database(&self, sql: String) -> serde_json::Value {
        serde_json::json!({"query": sql, "rows": []})
    }

    #[strands::tool(description = "Insert a record into a table.")]
    async fn insert_record(&self, table: String, data: serde_json::Value) -> String {
        format!("Inserted {data} into {table}")
    }
}

fn database_agent() -> strands::Result<strands::Agent> {
    let database = std::sync::Arc::new(DatabaseTools);
    strands::Agent::builder().tools(database.tools()).build()
}
// --8<-- [end:class_tools]

// --8<-- [start:effective_description]
#[strands::tool(description = "Search the product database for items matching the query string.\n\
\n\
Use this tool when you need to find detailed product information based on keywords,\n\
product names, or categories. The search is case-insensitive and supports fuzzy\n\
matching to handle typos and variations in search terms.\n\
\n\
This tool connects to the enterprise product catalog database and performs a semantic\n\
search across all product fields, providing comprehensive results with all available\n\
product metadata.\n\
\n\
Example response:\n\
[\n\
  {\n\
    \"id\": \"P12345\",\n\
    \"name\": \"Ultra Comfort Running Shoes\",\n\
    \"description\": \"Lightweight running shoes with...\",\n\
    \"price\": 89.99,\n\
    \"category\": [\"Footwear\", \"Athletic\", \"Running\"]\n\
  }\n\
]\n\
\n\
Notes:\n\
- This tool only searches the product catalog and does not provide inventory or availability information\n\
- Results are cached for 15 minutes to improve performance\n\
- The search index updates every 6 hours, so very recent products may not appear\n\
- For real-time inventory status, use a separate inventory check tool")]
async fn search_database(query: String, max_results: Option<u32>) -> serde_json::Value {
    serde_json::json!({
        "query": query,
        "maxResults": max_results.unwrap_or(10),
        "products": []
    })
}
// --8<-- [end:effective_description]

fn main() {}
