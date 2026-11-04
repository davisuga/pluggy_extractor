# Optional Google Drive copy

The existing S3 export and retention are unchanged. After a successful S3 sync,
the workflow can update one private `pluggy-latest.txt` file containing all five
JSONL datasets and the source run timestamp. It replaces the current contents;
S3 remains the historical archive. Google Drive may retain prior file revisions.
The assistant can read this text file through your separately connected Drive
account; indexing and updates may take time. Check the run timestamp before use.

## One-time setup (user action)

Leave `GOOGLE_DRIVE_ENABLED` unset until you explicitly consent to uploading your
banking exports to the selected Google account and file. No data is uploaded by
merging this change alone. This repository does not create credentials, files,
or sharing permissions.

1. Enable the Google Drive API in your own Google Cloud project and configure an
   OAuth client for your personal use. Authorize only
   `https://www.googleapis.com/auth/drive.file` with offline access. Obtain a
   refresh token privately; never paste it into chat, source control, or logs.
   An external OAuth app in Testing may issue refresh tokens that expire after
   seven days; configure the consent screen appropriately for ongoing use.
2. Using that same OAuth app, create a private `text/plain` file named
   `pluggy-latest.txt` in your Drive. The per-file scope cannot access arbitrary
   existing files unless they were created or explicitly authorized for that app.
   Use the account connected to the assistant. Verify the destination has no
   unintended sharing. Do not use a service account for personal My Drive storage.
3. In GitHub Actions secrets, add `GOOGLE_DRIVE_CLIENT_ID`,
   `GOOGLE_DRIVE_CLIENT_SECRET`, `GOOGLE_DRIVE_REFRESH_TOKEN`, and
   `GOOGLE_DRIVE_FILE_ID` (the ID of that exact text file).
4. After confirming the destination and consenting to the transfer, set the
   repository Actions variable `GOOGLE_DRIVE_ENABLED` to exactly `true`.
   The next scheduled sync (every six hours) will upload the fresh export. You
   can also run the existing workflow manually after setup.

Disable future copies by removing the variable or setting it to `false`, and
revoke the OAuth grant when no longer needed. No broad Drive scope is required.
The uploader never changes sharing, creates files, or deletes files.

## Failure behavior and validation

An enabled upload failure fails the workflow after S3 has completed; its previous
Drive contents remain available and may be stale. There is no automatic retry of
the extractor. Exports above 4 MiB, malformed JSONL, missing datasets, old local
runs, and non-text destinations are rejected. The current export includes all
fields supplied by the extractor, including sensitive account and transaction
details. No export is attached as a public GitHub artifact or printed by this
uploader. The text copy is a snapshot, not an incremental ledger.

Run offline checks using synthetic data:

```sh
python3 -m unittest discover -s scripts -p 'test_*.py'
```

References: [per-file scope](https://developers.google.com/workspace/drive/api/guides/api-specific-auth),
[update file content](https://developers.google.com/workspace/drive/api/guides/manage-uploads),
[refresh token expiration](https://developers.google.com/identity/protocols/oauth2#expiration).
