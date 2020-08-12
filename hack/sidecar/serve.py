#!/usr/bin/env python3

"""
Dummy service that will provide headers to add to requests.

This will be called by our filter.

Installed as a 'sidecar' in the hack setup but could as easily be an external
centralised service.
"""

from http.server import SimpleHTTPRequestHandler
from socketserver import TCPServer
import json
import uuid


class GetHandler(SimpleHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.end_headers()
        self.wfile.write(
            bytes(
                json.dumps(
                    {
                        "Authorization": "Bearer " + uuid.uuid4().hex,
                        "X-AnotherSecretHeader": "Hiya Pal",
                    }
                ),
                encoding="utf8",
            )
        )


Handler = GetHandler
httpd = TCPServer(("", 8081), Handler)

httpd.serve_forever()
