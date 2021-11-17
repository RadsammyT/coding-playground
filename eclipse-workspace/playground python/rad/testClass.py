'''
Created on Nov 6, 2021

@author: RadsammyT
'''

class Test:
    def __init__(self, piss):
        self.piss = piss
        
    def getPiss(self):
        return self.piss
    def setPiss(self, piss):
        self.piss = piss
    def __str__(self):
        return str(self.piss)
    