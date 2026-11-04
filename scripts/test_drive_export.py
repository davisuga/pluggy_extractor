import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import drive_export as export


class DriveExportTests(unittest.TestCase):
    def test_snapshot_preserves_nested_json_and_empty_sections(self):
        with tempfile.TemporaryDirectory() as directory:
            run = Path(directory) / '1700000000'
            run.mkdir()
            for name in export.FILES:
                (run / (name + '.jsonl')).write_text('', encoding='utf-8')
            (run / 'transactions.jsonl').write_text('{"transaction":{"id":"synthetic"}}\n')
            payload = export.snapshot(run).decode()
            self.assertIn('2023-11-14', payload)
            self.assertIn('"id":"synthetic"', payload)
            self.assertIn('=== investments ===', payload)
            (run / 'accounts.jsonl').write_text('invalid json')
            with self.assertRaises(ValueError):
                export.snapshot(run)

    def test_disabled_never_contacts_google(self):
        with patch.dict(export.os.environ, {}, clear=True), patch.object(export, 'request') as request:
            export.main()
            request.assert_not_called()

    def test_upload_updates_only_configured_file(self):
        config = dict(GOOGLE_DRIVE_FILE_ID='synthetic-id', GOOGLE_DRIVE_CLIENT_ID='client',
                      GOOGLE_DRIVE_CLIENT_SECRET='secret', GOOGLE_DRIVE_REFRESH_TOKEN='refresh')
        responses = [{'access_token': 'token'},
                     {'mimeType': 'text/plain', 'capabilities': {'canEdit': True}},
                     {'id': 'synthetic-id'}]
        with patch.object(export, 'request', side_effect=responses) as request:
            export.publish(b'synthetic export', config)
            self.assertEqual(request.call_count, 3)
            call = request.call_args.args
            self.assertEqual(call[3], 'PATCH')
            self.assertIn('/synthetic-id?', call[0])
            self.assertEqual(call[1], b'synthetic export')
        responses[1] = {'mimeType': 'application/vnd.google-apps.document'}
        with patch.object(export, 'request', side_effect=responses) as request:
            with self.assertRaises(ValueError):
                export.publish(b'synthetic export', config)
            self.assertEqual(request.call_count, 2)


if __name__ == '__main__':
    unittest.main()
