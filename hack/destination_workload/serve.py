#!/usr/bin/env python3

"""
Dummy destination workload that echos any headers it receives

This will be called by our dummy source workload, with packets being augmented
by our Envoy filter.
"""

from http.server import SimpleHTTPRequestHandler
from socketserver import TCPServer


class GetHandler(SimpleHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200, self.headers)
        for h in self.headers:
            self.send_header(h, self.headers[h])
        self.end_headers()


Handler = GetHandler
httpd = TCPServer(("", 8080), Handler)

httpd.serve_forever()
