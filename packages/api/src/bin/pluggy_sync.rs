use std::{
    collections::{HashMap, HashSet},
    env,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
    time::SystemTime,
};

use anyhow::{Context, Result};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    config::Builder as S3ConfigBuilder,
    primitives::ByteStream,
    types::{Delete, ObjectIdentifier},
    Client as S3Client,
};
use tokio::fs;

use api::pluggy_sync::{self, JsonlFileSink, SyncOptions, SyncSink};

#[tokio::main]
async fn main() -> Result<()> {
    let client_id = env::var("PLUGGY_CLIENT_ID")
        .context("PLUGGY_CLIENT_ID is required")
        .and_then(|val| uuid::Uuid::parse_str(&val).context("PLUGGY_CLIENT_ID must be a UUID"))?;
    let client_secret = env::var("PLUGGY_CLIENT_SECRET")
        .context("PLUGGY_CLIENT_SECRET is required")?;

    let item_ids = env::var("PLUGGY_ITEM_IDS")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "60d322ef-c24f-4ddf-a2b0-4197f31bfd07".to_string())
        .split(',')
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();

    let run_id = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .context("system time before unix epoch")?
        .as_secs();
    let base_dir = PathBuf::from(format!("data/pluggy_sync/{run_id}"));
    fs::create_dir_all(&base_dir)
        .await
        .context("create base output directory")?;

    let api_key = pluggy_sync::fetch_api_key(client_id, client_secret).await?;
    let config = Arc::new(pluggy_sync::with_api_key(api_key));

    let sink = Arc::new(JsonlFileSink::new(base_dir.clone()).await?);

    let options = SyncOptions {
        page_size: env::var("PLUGGY_PAGE_SIZE")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(500.0),
        max_concurrency: env::var("PLUGGY_MAX_CONCURRENCY")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(4),
        pause_between_pages: Duration::from_millis(
            env::var("PLUGGY_PAUSE_MS")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(150),
        ),
        from: env::var("PLUGGY_FROM").ok(),
        to: env::var("PLUGGY_TO").ok(),
        created_at_from: env::var("PLUGGY_CREATED_AT_FROM").ok(),
    };
    let stats = pluggy_sync::sync_all(config, sink.clone(), item_ids, options).await?;

    sink.finish().await?;

    upload_dir_to_s3(&base_dir).await?;
    prune_local_runs(&base_dir, max_runs_from_env()).await?;

    println!(
        "sync complete: accounts={}, account_details={}, transactions={}, investments={}, investment_transactions={}",
        stats.accounts,
        stats.account_details,
        stats.transactions,
        stats.investments,
        stats.investment_transactions
    );

    Ok(())
}

async fn upload_dir_to_s3(base_dir: &PathBuf) -> Result<()> {
    let bucket = env::var("S3_BUCKET").context("S3_BUCKET is required")?;
    let prefix = env::var("S3_PREFIX").unwrap_or_else(|_| "pluggy_sync".to_string());
    let region = env::var("S3_REGION").unwrap_or_else(|_| "us-west-001".to_string());
    let endpoint = env::var("S3_ENDPOINT")
        .ok()
        .and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                None
            } else if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
                Some(trimmed)
            } else {
                Some(format!("https://{trimmed}"))
            }
        });

    let mut config_loader =
        aws_config::defaults(BehaviorVersion::latest()).region(Region::new(region));
    if let Some(endpoint) = endpoint {
        config_loader = config_loader.endpoint_url(endpoint);
    }
    let shared_config = config_loader.load().await;

    let s3_config = S3ConfigBuilder::from(&shared_config)
        .force_path_style(true)
        .build();
    let client = S3Client::from_conf(s3_config);

    let mut entries = fs::read_dir(base_dir)
        .await
        .with_context(|| format!("read dir {base_dir:?}"))?;

    let run_name = base_dir
        .file_name()
        .and_then(|name| name.to_str())
        .context("base dir name not valid UTF-8")?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .context("file name not valid UTF-8")?;

        let key = format!("{prefix}/{run_name}/{file_name}");
        let body = ByteStream::from_path(&path)
            .await
            .with_context(|| format!("open file {path:?}"))?;

        client
            .put_object()
            .bucket(&bucket)
            .key(&key)
            .body(body)
            .send()
            .await
            .with_context(|| format!("upload {key}"))?;
    }

    println!("uploaded to s3://{bucket}/{prefix}/{run_name}");

    prune_old_s3_runs(&client, &bucket, &prefix, max_runs_from_env()).await?;

    Ok(())
}

fn max_runs_from_env() -> usize {
    env::var("PLUGGY_MAX_RUNS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(30)
}

async fn prune_local_runs(base_dir: &Path, keep: usize) -> Result<()> {
    let parent = base_dir
        .parent()
        .context("local base dir missing parent")?;
    let mut entries = fs::read_dir(parent)
        .await
        .with_context(|| format!("read local dir {parent:?}"))?;

    let mut runs = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let name = match path.file_name().and_then(|v| v.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };

        if let Ok(run_id) = name.parse::<u64>() {
            runs.push((run_id, path));
        }
    }

    runs.sort_by_key(|(run_id, _)| *run_id);
    if runs.len() <= keep {
        return Ok(());
    }

    let remove_count = runs.len() - keep;
    for (_, path) in runs.into_iter().take(remove_count) {
        fs::remove_dir_all(&path)
            .await
            .with_context(|| format!("remove local run {path:?}"))?;
    }

    Ok(())
}

async fn prune_old_s3_runs(
    client: &S3Client,
    bucket: &str,
    prefix: &str,
    keep: usize,
) -> Result<()> {
    let mut continuation = None;
    let mut run_keys: HashMap<String, Vec<String>> = HashMap::new();
    let normalized_prefix = prefix.trim_matches('/');
    let prefix_path = format!("{normalized_prefix}/");

    loop {
        let mut req = client.list_objects_v2().bucket(bucket).prefix(&prefix_path);
        if let Some(token) = continuation.take() {
            req = req.continuation_token(token);
        }
        let resp = req.send().await?;

        for obj in resp.contents() {
            let Some(key) = obj.key() else {
                continue;
            };
            let Some(run_id) = extract_run_id(&prefix_path, key) else {
                continue;
            };
            run_keys
                .entry(run_id)
                .or_default()
                .push(key.to_string());
        }

        if resp.is_truncated().unwrap_or(false) {
            continuation = resp.next_continuation_token().map(|v| v.to_string());
        } else {
            break;
        }
    }

    if run_keys.len() <= keep {
        return Ok(());
    }

    let mut numeric_runs: Vec<(u64, String)> = run_keys
        .keys()
        .filter_map(|run| run.parse::<u64>().ok().map(|id| (id, run.clone())))
        .collect();
    numeric_runs.sort_by_key(|(id, _)| *id);

    if numeric_runs.len() <= keep {
        return Ok(());
    }

    let keep_set: HashSet<String> = numeric_runs
        .into_iter()
        .rev()
        .take(keep)
        .map(|(_, run)| run)
        .collect();

    let mut delete_keys = Vec::new();
    for (run_id, keys) in run_keys {
        if keep_set.contains(&run_id) {
            continue;
        }
        delete_keys.extend(keys);
    }

    for chunk in delete_keys.chunks(1000) {
        let objects = chunk
            .iter()
            .map(|key| ObjectIdentifier::builder().key(key).build())
            .collect::<Result<Vec<_>, _>>()
            .context("build delete object identifiers")?;

        let delete = Delete::builder()
            .set_objects(Some(objects))
            .build()
            .context("build delete payload")?;
        client
            .delete_objects()
            .bucket(bucket)
            .delete(delete)
            .send()
            .await
            .context("delete old s3 runs")?;
    }

    Ok(())
}

fn extract_run_id(prefix_path: &str, key: &str) -> Option<String> {
    let rest = key.strip_prefix(prefix_path)?;
    let (run_id, _) = rest.split_once('/')?;
    if run_id.is_empty() {
        return None;
    }
    Some(run_id.to_string())
}
