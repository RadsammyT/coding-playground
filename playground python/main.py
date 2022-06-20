from rad.collatz import *
from time import sleep
from rad.timerClass import *
import PySimpleGUI as pg
import rad.ShitShuffler
import itertools as it
from rad.textbox import *
import os

def MainOne():
    t = Timer()
    t.start()
    test = [0,1]
    for i in range(50):
        test.append(test[i]+test[i+1])
    print(test)
    rad.textbox.run( " test", 0.5)
    t.stop()
    print(t.getElapsed())

def MainTwo():
    rad.ShitShuffler.run(1,1)

def MainThree():
    
    lay = [[pg.Text('Length of list | repeat')], 
           [pg.InputText(size=(10,1), key='input'), pg.InputText(size=(10,1), key='repeat')],
           [pg.Button('Submit')],
           [pg.Text('test', key = '-OUTPUT-')]]
    win = pg.Window('Collatz', layout=lay)
    while True:
        event, values = win.read()
        
        if event == 'Submit':
            win['-OUTPUT-'].update(str(
                rad.ShitShuffler.run(values['input'], values['repeat'])) + " failed")
        elif event == pg.WIN_CLOSED:
            break

def MainFour():
    for i in it.permutations(range(3)):
        print(i)

def MainFive():
    os.system("echo test")

dict = [MainOne, MainTwo, MainThree, MainFour, MainFive]
for i,j in enumerate(dict):
    print(str(i) + " " + str(j.__name__))
    

inp = input("Main_: ")
dict[int(inp)]()