'''
navigate terminal to the folder test.py is in and run this module
'''
import pygame as pg
import sys

pg.init()

screen = pg.display.set_mode((640, 480))
image = pg.image.load("./test.jpg") 
imgX = 20
imgY = 20

run = True

pg.key.set_repeat(1, 50)
while run:
    for event in pg.event.get():
        if event.type == pg.QUIT:
            run = False
            pg.quit()
            sys.exit()
        # move image if key is pressed, must be continuous
        if event.type == pg.KEYDOWN:
            if event.key == pg.K_LEFT:
                imgX -= 10
            if event.key == pg.K_RIGHT:
                imgX += 10
            if event.key == pg.K_UP:
                imgY -= 10
            if event.key == pg.K_DOWN:
                imgY += 10
            


    screen.fill((0, 0, 0))
    screen.blit(image, (imgX, imgY))
                
    pg.display.flip()
pg.quit()
sys.exit()


