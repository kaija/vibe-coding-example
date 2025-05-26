#!/usr/bin/env python3
"""
Simple HTTP server for serving the Notes App frontend.
This script provides a quick way to serve the static files for development.
"""

import http.server
import socketserver
import webbrowser
import os
import sys
from pathlib import Path

# Configuration
PORT = 3000
HOST = 'localhost'

class CustomHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    """Custom handler to serve index.html for SPA routing and add CORS headers"""
    
    def end_headers(self):
        # Add CORS headers for development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type, Authorization')
        super().end_headers()
    
    def do_GET(self):
        # Serve index.html for root path
        if self.path == '/':
            self.path = '/index.html'
        return super().do_GET()

def main():
    # Change to the frontend directory
    frontend_dir = Path(__file__).parent
    os.chdir(frontend_dir)
    
    # Create server
    with socketserver.TCPServer((HOST, PORT), CustomHTTPRequestHandler) as httpd:
        server_url = f"http://{HOST}:{PORT}"
        
        print("=" * 60)
        print("🚀 Notes App Frontend Server")
        print("=" * 60)
        print(f"📂 Serving files from: {frontend_dir}")
        print(f"🌐 Server running at: {server_url}")
        print(f"📱 Open in browser: {server_url}")
        print("=" * 60)
        print("📋 Make sure the backend API is running on http://localhost:8000")
        print("⏹️  Press Ctrl+C to stop the server")
        print("=" * 60)
        
        # Try to open browser automatically
        try:
            webbrowser.open(server_url)
            print("🌐 Browser opened automatically")
        except Exception as e:
            print(f"⚠️  Could not open browser automatically: {e}")
            print(f"   Please open {server_url} manually")
        
        print()
        
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n👋 Server stopped by user")
            sys.exit(0)

if __name__ == "__main__":
    main()