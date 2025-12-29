INSTALL httpfs;
LOAD httpfs;

SET s3_endpoint = getenv('S3_ENDPOINT');
SET s3_access_key_id = getenv('AWS_ACCESS_KEY_ID');
SET s3_secret_access_key = getenv('AWS_SECRET_ACCESS_KEY');
SET s3_region = getenv('S3_REGION');
SET s3_url_style = 'path';

SELECT COUNT(*) AS transactions
FROM read_json_auto('s3://myfinance-data/pluggy_sync/1766966566/transactions.jsonl');

SELECT
  transaction.id,
  transaction.description,
  transaction.amount,
  transaction.date
FROM read_json_auto('s3://myfinance-data/pluggy_sync/1766966566/transactions.jsonl')
LIMIT 5;
