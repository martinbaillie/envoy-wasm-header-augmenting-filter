#!/usr/bin/env python3

"""

Dummy source workload that simply returns 200 (so that liveness/readiness
probes continue to work).

"""

from http.server import SimpleHTTPRequestHandler
from socketserver import TCPServer


class GetHandler(SimpleHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200, "OK")
        self.send_header("Content-type", "text/plain")
        self.end_headers()


Handler = GetHandler
httpd = TCPServer(("", 8080), Handler)

httpd.serve_forever()
