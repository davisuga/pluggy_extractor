"""Publish the current completed export to one explicitly configured Drive file."""
import datetime
import json
import os
from pathlib import Path
import re
import sys
import urllib.parse
import urllib.request

FILES = ('accounts', 'account_details', 'transactions', 'investments',
         'investment_transactions')
MAX_BYTES = 4 * 1024 * 1024


def snapshot(run):
    parts = ['Pluggy banking export\n',
             'Run UTC: ' + datetime.datetime.fromtimestamp(
                 int(run.name), datetime.timezone.utc).isoformat() + '\n',
             'Source format: JSON Lines. Empty sections mean no records.\n']
    total = 0
    for name in FILES:
        path = run / (name + '.jsonl')
        if path.is_symlink() or not path.is_file():
            raise ValueError('Missing or unsafe export file')
        total += path.stat().st_size
        if total > MAX_BYTES:
            raise ValueError('Export exceeds 4 MiB limit')
        content = path.read_text(encoding='utf-8')
        for line in content.splitlines():
            if line.strip():
                json.loads(line)
        parts.extend(['\n=== ' + name + ' ===\n', content])
    return ''.join(parts).encode('utf-8')


def request(url, data=None, headers=None, method=None):
    # Never print response bodies: OAuth errors and exports may contain secrets.
    req = urllib.request.Request(url, data=data, headers=headers or {}, method=method)
    with urllib.request.urlopen(req, timeout=120) as response:
        return json.load(response)


def publish(payload, config):
    file_id = config['GOOGLE_DRIVE_FILE_ID'].strip()
    if not re.fullmatch(r'[A-Za-z0-9_-]+', file_id):
        raise ValueError('Invalid Drive file ID')
    token = request('https://oauth2.googleapis.com/token', urllib.parse.urlencode({
        'client_id': config['GOOGLE_DRIVE_CLIENT_ID'],
        'client_secret': config['GOOGLE_DRIVE_CLIENT_SECRET'],
        'refresh_token': config['GOOGLE_DRIVE_REFRESH_TOKEN'],
        'grant_type': 'refresh_token',
    }).encode(), {'Content-Type': 'application/x-www-form-urlencoded'})['access_token']
    headers = {'Authorization': 'Bearer ' + token}
    metadata = request('https://www.googleapis.com/drive/v3/files/' + file_id
                       + '?fields=id,mimeType,trashed,capabilities(canEdit)', headers=headers)
    if (metadata.get('trashed') or metadata.get('mimeType') != 'text/plain'
            or not metadata.get('capabilities', {}).get('canEdit')):
        raise ValueError('Destination must be an editable, non-trashed text/plain file')
    request('https://www.googleapis.com/upload/drive/v3/files/' + file_id
            + '?uploadType=media&fields=id', payload,
            dict(headers, **{'Content-Type': 'text/plain; charset=utf-8'}), 'PATCH')


def main():
    if os.environ.get('GOOGLE_DRIVE_ENABLED') != 'true':
        print('Drive export disabled')
        return
    keys = ('GOOGLE_DRIVE_FILE_ID', 'GOOGLE_DRIVE_CLIENT_ID',
            'GOOGLE_DRIVE_CLIENT_SECRET', 'GOOGLE_DRIVE_REFRESH_TOKEN')
    config = {key: os.environ[key] for key in keys}
    root = Path('data/pluggy_sync')
    runs = [p for p in root.iterdir()
            if p.name.isascii() and p.name.isdigit() and p.is_dir() and not p.is_symlink()]
    run = max(runs, key=lambda p: int(p.name))
    if abs(datetime.datetime.now().timestamp() - int(run.name)) > 3600:
        raise ValueError('No fresh export found')
    publish(snapshot(run), config)
    print('Drive snapshot updated')


if __name__ == '__main__':
    try:
        main()
    except Exception:
        print('Drive export failed; check configuration and export validity. S3 sync already completed.',
              file=sys.stderr)
        sys.exit(1)
