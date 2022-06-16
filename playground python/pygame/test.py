'''
navigate terminal to the folder test.py is in and run this module
'''
import pygame as pg
import sys

pg.init()

screen = pg.display.set_mode((640, 480))
image = pg.image.load("./playground python/pygame/test.jpg") 
imgX = 20
imgY = 20
text = "hello world"
textX = 20
textY = 20
font = pg.font.SysFont("Arial", 20)
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
            if event.key == pg.K_a:
                imgX -= 10
            if event.key == pg.K_d:
                imgX += 10
            if event.key == pg.K_w:
                imgY -= 10
            if event.key == pg.K_s:
                imgY += 10
            


    screen.fill((0, 0, 0))
    screen.blit(image, (imgX, imgY))
    #text
    
    text = font.render("text", True, (255, 255, 255))
    screen.blit(text, (textX, textY))
    pg.display.flip()
pg.quit()
print(sys.exit())


