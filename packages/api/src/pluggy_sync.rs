use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use async_trait::async_trait;
use pluggy_rust::{
    apis::{account_api, auth_api, configuration, investment_api, transaction_api},
    models,
};
use serde::Serialize;
use tokio::{
    fs::{self, File},
    io::{AsyncWriteExt, BufWriter},
    sync::{Mutex, Semaphore},
    task::JoinSet,
    time::sleep,
};

#[derive(Clone, Debug)]
pub struct SyncOptions {
    pub page_size: f64,
    pub max_concurrency: usize,
    pub pause_between_pages: Duration,
    pub from: Option<String>,
    pub to: Option<String>,
    pub created_at_from: Option<String>,
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            page_size: 500.0,
            max_concurrency: 4,
            pause_between_pages: Duration::from_millis(150),
            from: None,
            to: None,
            created_at_from: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SyncStats {
    pub accounts: usize,
    pub account_details: usize,
    pub transactions: usize,
    pub investments: usize,
    pub investment_transactions: usize,
}

impl SyncStats {
    fn add(&mut self, other: SyncStats) {
        self.accounts += other.accounts;
        self.account_details += other.account_details;
        self.transactions += other.transactions;
        self.investments += other.investments;
        self.investment_transactions += other.investment_transactions;
    }
}

#[async_trait]
pub trait SyncSink: Send + Sync {
    async fn write_accounts(&self, accounts: &[models::Account]) -> Result<()>;
    async fn write_account_details(&self, account: &models::Account) -> Result<()>;
    async fn write_transactions(
        &self,
        item_id: &str,
        account_id: &str,
        transactions: &[models::Transaction],
    ) -> Result<()>;
    async fn write_investments(&self, investments: &[models::Investment]) -> Result<()>;
    async fn write_investment_transactions(
        &self,
        item_id: &str,
        investment_id: &str,
        transactions: &[models::InvestmentTransaction],
    ) -> Result<()>;
    async fn finish(&self) -> Result<()>;
}

pub struct JsonlFileSink {
    base_dir: PathBuf,
    accounts: Mutex<BufWriter<File>>,
    account_details: Mutex<BufWriter<File>>,
    transactions: Mutex<BufWriter<File>>,
    investments: Mutex<BufWriter<File>>,
    investment_transactions: Mutex<BufWriter<File>>,
}

impl JsonlFileSink {
    pub async fn new(base_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&base_dir)
            .await
            .with_context(|| format!("create output dir {base_dir:?}"))?;

        let accounts = File::create(base_dir.join("accounts.jsonl"))
            .await
            .context("open accounts.jsonl")?;
        let account_details = File::create(base_dir.join("account_details.jsonl"))
            .await
            .context("open account_details.jsonl")?;
        let transactions = File::create(base_dir.join("transactions.jsonl"))
            .await
            .context("open transactions.jsonl")?;
        let investments = File::create(base_dir.join("investments.jsonl"))
            .await
            .context("open investments.jsonl")?;
        let investment_transactions = File::create(base_dir.join("investment_transactions.jsonl"))
            .await
            .context("open investment_transactions.jsonl")?;

        Ok(Self {
            base_dir,
            accounts: Mutex::new(BufWriter::new(accounts)),
            account_details: Mutex::new(BufWriter::new(account_details)),
            transactions: Mutex::new(BufWriter::new(transactions)),
            investments: Mutex::new(BufWriter::new(investments)),
            investment_transactions: Mutex::new(BufWriter::new(investment_transactions)),
        })
    }

    pub fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }

    async fn write_json_lines<T: Serialize>(
        writer: &Mutex<BufWriter<File>>,
        values: impl IntoIterator<Item = T>,
    ) -> Result<()> {
        let mut buf = Vec::new();
        for value in values {
            let mut line = serde_json::to_vec(&value)?;
            buf.append(&mut line);
            buf.push(b'\n');
        }

        let mut guard = writer.lock().await;
        guard.write_all(&buf).await?;
        Ok(())
    }

    async fn flush_writer(writer: &Mutex<BufWriter<File>>) -> Result<()> {
        let mut guard = writer.lock().await;
        guard.flush().await?;
        Ok(())
    }
}

#[async_trait]
impl SyncSink for JsonlFileSink {
    async fn write_accounts(&self, accounts: &[models::Account]) -> Result<()> {
        Self::write_json_lines(&self.accounts, accounts.to_vec()).await
    }

    async fn write_account_details(&self, account: &models::Account) -> Result<()> {
        Self::write_json_lines(&self.account_details, [account.clone()]).await
    }

    async fn write_transactions(
        &self,
        item_id: &str,
        account_id: &str,
        transactions: &[models::Transaction],
    ) -> Result<()> {
        let records = transactions.iter().map(|transaction| TransactionRecord {
            item_id,
            account_id,
            transaction,
        });

        let mut buf = Vec::new();
        for record in records {
            let mut line = serde_json::to_vec(&record)?;
            buf.append(&mut line);
            buf.push(b'\n');
        }

        let mut guard = self.transactions.lock().await;
        guard.write_all(&buf).await?;
        Ok(())
    }

    async fn write_investments(&self, investments: &[models::Investment]) -> Result<()> {
        Self::write_json_lines(&self.investments, investments.to_vec()).await
    }

    async fn write_investment_transactions(
        &self,
        item_id: &str,
        investment_id: &str,
        transactions: &[models::InvestmentTransaction],
    ) -> Result<()> {
        let records = transactions.iter().map(|transaction| InvestmentTransactionRecord {
            item_id,
            investment_id,
            transaction,
        });

        let mut buf = Vec::new();
        for record in records {
            let mut line = serde_json::to_vec(&record)?;
            buf.append(&mut line);
            buf.push(b'\n');
        }

        let mut guard = self.investment_transactions.lock().await;
        guard.write_all(&buf).await?;
        Ok(())
    }

    async fn finish(&self) -> Result<()> {
        Self::flush_writer(&self.accounts).await?;
        Self::flush_writer(&self.account_details).await?;
        Self::flush_writer(&self.transactions).await?;
        Self::flush_writer(&self.investments).await?;
        Self::flush_writer(&self.investment_transactions).await?;
        Ok(())
    }
}

#[derive(Serialize)]
struct TransactionRecord<'a> {
    item_id: &'a str,
    account_id: &'a str,
    transaction: &'a models::Transaction,
}

#[derive(Serialize)]
struct InvestmentTransactionRecord<'a> {
    item_id: &'a str,
    investment_id: &'a str,
    transaction: &'a models::InvestmentTransaction,
}

pub async fn fetch_api_key(client_id: uuid::Uuid, client_secret: String) -> Result<String> {
    let config = configuration::Configuration::default();
    let response = auth_api::auth_create(
        &config,
        models::AuthRequest::new(client_id, client_secret),
    )
    .await?;
    Ok(response.api_key)
}

pub fn with_api_key(api_key: String) -> configuration::Configuration {
    let mut config = configuration::Configuration::default();
    config.api_key = Some(configuration::ApiKey {
        prefix: None,
        key: api_key,
    });
    config
}

pub async fn sync_all(
    config: Arc<configuration::Configuration>,
    sink: Arc<dyn SyncSink>,
    item_ids: Vec<String>,
    options: SyncOptions,
) -> Result<SyncStats> {
    let semaphore = Arc::new(Semaphore::new(options.max_concurrency));
    let mut set = JoinSet::new();

    for item_id in item_ids {
        let config = config.clone();
        let sink = sink.clone();
        let semaphore = semaphore.clone();
        let options = options.clone();

        set.spawn(async move {
            sync_item(config, sink, semaphore, &item_id, &options).await
        });
    }

    let mut total = SyncStats::default();
    while let Some(result) = set.join_next().await {
        total.add(result??);
    }

    Ok(total)
}

async fn sync_item(
    config: Arc<configuration::Configuration>,
    sink: Arc<dyn SyncSink>,
    semaphore: Arc<Semaphore>,
    item_id: &str,
    options: &SyncOptions,
) -> Result<SyncStats> {
    let accounts_response = account_api::accounts_list(&config, item_id, None).await?;
    let accounts = accounts_response.results.unwrap_or_default();
    if !accounts.is_empty() {
        sink.write_accounts(&accounts).await?;
    }

    let mut stats = SyncStats {
        accounts: accounts.len(),
        ..Default::default()
    };

    let investments = fetch_investments(&config, item_id, options).await?;
    if !investments.is_empty() {
        sink.write_investments(&investments).await?;
    }
    stats.investments = investments.len();

    let mut set = JoinSet::new();
    for account in accounts {
        let config = config.clone();
        let sink = sink.clone();
        let semaphore = semaphore.clone();
        let options = options.clone();

        set.spawn(async move {
            let _permit = semaphore.acquire_owned().await?;
            sync_account(config, sink, account, &options).await
        });
    }

    for investment in investments {
        let config = config.clone();
        let sink = sink.clone();
        let semaphore = semaphore.clone();
        let options = options.clone();

        set.spawn(async move {
            let _permit = semaphore.acquire_owned().await?;
            sync_investment_transactions(config, sink, investment, &options).await
        });
    }

    while let Some(result) = set.join_next().await {
        stats.add(result??);
    }

    Ok(stats)
}

async fn sync_account(
    config: Arc<configuration::Configuration>,
    sink: Arc<dyn SyncSink>,
    account: models::Account,
    options: &SyncOptions,
) -> Result<SyncStats> {
    let account_id = account.id.clone();
    let item_id = account.item_id.to_string();

    let details = account_api::accounts_retrieve(&config, &account_id).await?;
    sink.write_account_details(&details).await?;

    let mut page = 1.0;
    let mut stats = SyncStats {
        account_details: 1,
        ..Default::default()
    };

    loop {
        let page_resp = transaction_api::transactions_list(
            &config,
            &account_id,
            None,
            options.from.clone(),
            options.to.clone(),
            Some(options.page_size),
            Some(page),
            None,
            options.created_at_from.clone(),
        )
        .await?;

        if page_resp.results.is_empty() {
            break;
        }

        sink.write_transactions(&item_id, &account_id, &page_resp.results)
            .await?;
        stats.transactions += page_resp.results.len();

        if page >= page_resp.total_pages {
            break;
        }
        page += 1.0;

        if !options.pause_between_pages.is_zero() {
            sleep(options.pause_between_pages).await;
        }
    }

    Ok(stats)
}

async fn fetch_investments(
    config: &configuration::Configuration,
    item_id: &str,
    options: &SyncOptions,
) -> Result<Vec<models::Investment>> {
    let mut investments = Vec::new();
    let mut page = 1.0;

    loop {
        let page_resp = investment_api::investments_list(
            config,
            item_id,
            None,
            Some(options.page_size),
            Some(page),
        )
        .await?;

        let mut results = page_resp.results.unwrap_or_default();
        if results.is_empty() {
            break;
        }
        investments.append(&mut results);

        let total_pages = page_resp.total_pages.unwrap_or(page);
        if page >= total_pages {
            break;
        }
        page += 1.0;

        if !options.pause_between_pages.is_zero() {
            sleep(options.pause_between_pages).await;
        }
    }

    Ok(investments)
}

async fn sync_investment_transactions(
    config: Arc<configuration::Configuration>,
    sink: Arc<dyn SyncSink>,
    investment: models::Investment,
    options: &SyncOptions,
) -> Result<SyncStats> {
    let investment_id = investment.id.clone();
    let item_id = investment.item_id.to_string();

    let mut page = 1.0;
    let mut stats = SyncStats::default();

    loop {
        let page_resp = investment_api::investment_transactions_list(
            &config,
            &investment_id,
            Some(options.page_size),
            Some(page),
        )
        .await?;

        if page_resp.results.is_empty() {
            break;
        }

        sink.write_investment_transactions(
            &item_id,
            &investment_id,
            &page_resp.results,
        )
        .await?;
        stats.investment_transactions += page_resp.results.len();

        if page >= page_resp.total_pages {
            break;
        }
        page += 1.0;

        if !options.pause_between_pages.is_zero() {
            sleep(options.pause_between_pages).await;
        }
    }

    Ok(stats)
}
