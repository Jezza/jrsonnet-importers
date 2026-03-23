use std::path::Path;

pub fn state() -> jrsonnet_evaluator::State {
    let state = jrsonnet_evaluator::State::default();

    let ctx = jrsonnet_stdlib::ContextInitializer::new(
        state.clone(),
        jrsonnet_evaluator::trace::PathResolver::FileName,
    );
    state.set_context_initializer(ctx);

    state
}

pub fn eval_str(state: &jrsonnet_evaluator::State, input: &str) -> anyhow::Result<String> {
    let val = state
        .evaluate_snippet("<chunk>", input)
        .map_err(|err| format_error(&err))?;

    manifest(val)
}

pub fn eval_str_as<T>(state: &jrsonnet_evaluator::State, input: &str) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let text = eval_str(state, input)?;

    parse_json(&text)
}

pub fn eval_file(state: &jrsonnet_evaluator::State, input: &Path) -> anyhow::Result<String> {
    let val = state.import(input).map_err(|err| format_error(&err))?;

    manifest(val)
}

pub fn eval_file_as<T>(state: &jrsonnet_evaluator::State, input: &Path) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let text = eval_file(state, input)?;

    parse_json(&text)
}

fn parse_json<T>(text: &str) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let value: serde_json::Value = serde_json::from_str(&text).expect("Unable to parse json");
    // @TODO jeremy - 02 Jan 2025: We do the round-trip in hopes that position info is easier to reason about.
    //  Don't know if it's still needed.
    let text = serde_json::to_string_pretty(&value).expect("Unable to format json as text.");

    let item = serde_json::from_str(&text)?;

    Ok(item)
}

fn manifest(val: jrsonnet_evaluator::Val) -> anyhow::Result<String> {
    let formatter = jrsonnet_evaluator::manifest::JsonFormat::minify(true);

    val.manifest(formatter).map_err(|err| format_error(&err))
}

fn format_error(error: &jrsonnet_evaluator::Error) -> anyhow::Error {
    use jrsonnet_evaluator::trace::TraceFormat;

    let format = jrsonnet_evaluator::trace::CompactFormat {
        resolver: jrsonnet_evaluator::trace::PathResolver::new_cwd_fallback(),
        max_trace: 20,
        padding: 4,
    };
    match format.format(error) {
        Ok(formatted) => anyhow::anyhow!("{}", formatted),
        Err(_) => anyhow::anyhow!("{}", error),
    }
}
