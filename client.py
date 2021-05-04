#!/usr/bin/env python3
"""websocket cmd client for actix/websocket-tcp-chat example."""
import argparse
import asyncio
import signal
import sys

import aiohttp

queue = asyncio.Queue()


async def start_client(url, loop):
    name = input('Please enter your name: ')

    ws = await aiohttp.ClientSession().ws_connect(url, autoclose=False, autoPing=False)

    def stdin_callback():
        line = sys.stdin.buffer.readline().decode('utf-8')
        if not line:
            loop.stop()
        else:
            # Queue.put is a corutine, so cant be called directly.
            asyncio.ensure_future(queue.put(ws.send_str(f"{name}: {line}")))

    loop.add_reader(sys.stdin, stdin_callback)

    async def dispatch():
        return False
