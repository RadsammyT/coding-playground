from rad.collatz import collatzRun
from rad.timerClass import Timer 
from rad.testClass import Test
from time import sleep
from rad.textbox import printLBL
t = Timer()
t.start()
print(collatzRun(9663, 3))

'''
grid = [1,2,3,
        4,5,6,
        7,8,9]

print(grid[0:3])
print(grid[3:6])
print(grid[6:9])

'''



printLBL(843,0.5)


pee = Test(34)
print(pee)


t.stop()
print(t.getElapsed())