"""
TODO
"""
import json
from bitstring import BitArray


URI = "ws://localhost:8080"

async def call_websocket(websocket, data: dict):
    """
    TODO
    """
    await websocket.send(json.dumps(data))
    results = await websocket.recv()
    return results
    


async def attach_device(websocket):
    """
    TODO
    """
    device = await get_devices_list(websocket)
    data = {
        "Opcode": "Attach",
        "Space": "SNES",
        "Operands": [f"{device}"]
    }

    print(data)

    # attach doesn't send a return code
    await websocket.send(json.dumps(data))


async def get_devices_list(websocket):
    """
    TODO
    """
    data = {
        "Opcode": "DeviceList",
        "Space": "SNES"
    }

    results = await call_websocket(websocket, data)
    try:
        device = json.loads(results)['Results'][0]
    except IndexError:
        print("No device connected")
        exit()

    return device


async def get_address(websocket, address, byte_length):
    """
    TODO
    """
    data = {
        "Opcode": "GetAddress",
        "Space": "SNES",
        "Operands": [f"{address}", f"{byte_length}"]
    }

    return await call_websocket(websocket, data)


async def get_inputs(websocket):
    """
    TODO
    """
    controller1_inputs_addr = "008B"
    input1 = await get_address(websocket, f'F5{controller1_inputs_addr}', 2)

    return BitArray(input1)

