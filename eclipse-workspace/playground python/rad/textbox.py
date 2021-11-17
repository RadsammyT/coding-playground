'''
Created on Nov 14, 2021

@author: RadsammyT
'''
from time import sleep

def printLBL(x, y):
    for i in str(x):
        print(i, end='')
        sleep(y)
    print()
    