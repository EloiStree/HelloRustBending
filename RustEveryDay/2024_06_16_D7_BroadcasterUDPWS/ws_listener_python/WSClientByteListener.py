import asyncio
import websockets
import time
import struct

websocket_address= 'ws://192.168.1.250:1237'
websocket_address= 'ws://127.0.0.1:1237'
websocket_address= 'ws://81.240.94.97:1237'

async def listen_to_websocket():
    global websocket_address
    while True:
        try:
            async with websockets.connect(websocket_address) as websocket:
                while True:
                    data = await websocket.recv()
                    print(f'Received data: {data}')
                    if(len(data)==4):
                        int_i = struct.unpack("<i", data)[0]
                        print(f"Integer:{int_i}")

        except Exception as e:
            print("Connection lost. Reconnecting in 5 seconds...")
            print(e)
            time.sleep(5)

asyncio.get_event_loop().run_until_complete(listen_to_websocket())