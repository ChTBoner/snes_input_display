"""
Super Metroid input controller
"""
from pathlib import PurePath
from sys import exit as sys_exit
import asyncio
import pygame
from websockets.client import connect
import websockets.exceptions
from QUsb2Snes import attach_device, get_inputs

CONTROLLER_IMAGE = pygame.image.load(PurePath("assets", "snes_controller.png"))
WIN = pygame.display.set_mode((CONTROLLER_IMAGE.get_width(), CONTROLLER_IMAGE.get_height()))

URI = "ws://localhost:8080"

PRESSED_COLOR = (255, 59, 0, 1)
DPAD_INPUT_SIZE = 40
ROUND_BUTTON_RADIUS = 20
SHOULDER_BUTTON_WIDTH = 5


def draw_window(inputs):
    """
    Draws the window.
    Arguments:
    Inputs: 16 bit BitArray from bitstring

    Inputs are checked and drawn if needed
    
    """
    WIN.blit(CONTROLLER_IMAGE, (0, 0))
    
    # A is pressed
    if inputs[0]:
        pygame.draw.circle(WIN, PRESSED_COLOR, (500, 132), ROUND_BUTTON_RADIUS)
    # X is pressed
    if inputs[1]:
        pygame.draw.circle(WIN, PRESSED_COLOR, (450, 88), ROUND_BUTTON_RADIUS)
    # B is pressed
    if inputs[8]:
        pygame.draw.circle(WIN, PRESSED_COLOR, (452, 165), ROUND_BUTTON_RADIUS)
    # Y is pressed
    if inputs[9]:
        pygame.draw.circle(WIN, PRESSED_COLOR, (403, 120), ROUND_BUTTON_RADIUS)

    # L is pressed
    if inputs[2]:
        pygame.draw.rect(WIN, PRESSED_COLOR, (128, 2, 77, SHOULDER_BUTTON_WIDTH))
    # R is pressed
    if inputs[3]:
        pygame.draw.rect(WIN, PRESSED_COLOR, (377, 2, 73, SHOULDER_BUTTON_WIDTH))

    # Select is pressed
    if inputs[10]:
        pygame.draw.polygon(WIN, PRESSED_COLOR, ((224, 152), (231, 163), (263, 142), (255, 131)))
    # Start is pressed
    if inputs[11]:
        pygame.draw.polygon(WIN, PRESSED_COLOR, ((280, 153), (287, 163), (320, 142), (311, 131)))
    
    # UP is pressed
    if inputs[12]:
        pygame.draw.rect(WIN, PRESSED_COLOR, (102, 66, DPAD_INPUT_SIZE, DPAD_INPUT_SIZE))
    # DOWN is pressed
    if inputs[13]:
        pygame.draw.rect(WIN, PRESSED_COLOR, (102, 144, DPAD_INPUT_SIZE, DPAD_INPUT_SIZE))
    # LEFT is pressed
    if inputs[14]:
        pygame.draw.rect(WIN, PRESSED_COLOR, (63, 105, DPAD_INPUT_SIZE, DPAD_INPUT_SIZE))
    # RIGHT is pressed
    if inputs[15]:
        pygame.draw.rect(WIN, PRESSED_COLOR, (141, 105, 40, DPAD_INPUT_SIZE))
    # pygame.draw.rect(WIN, PRESSED_COLOR, (40, 20, 40, 20))
    pygame.display.update()


async def main():
    """
    Main function
    Connects to QUsb2Snes and runs the main loop.
    """
    try:
        async with connect(URI) as qusb2snes:
            await attach_device(qusb2snes)
            run = True

            while run:
                for event in pygame.event.get():
                    if event.type == pygame.QUIT:
                        run = False
                inputs = await get_inputs(qusb2snes)
                draw_window(inputs)
    except OSError:
        sys_exit("Could not connect to QUsb2Snes")
    except websockets.exceptions.ConnectionClosed:
        sys_exit("Connection to Sd2Snes failed.")


if __name__ == "__main__":
    asyncio.run(main())
