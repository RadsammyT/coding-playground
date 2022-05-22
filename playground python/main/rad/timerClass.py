'''
Created on Nov 4, 2021

@author: RadsammyT
'''
import time
from time import perf_counter
# from pickle import NONE
 
class TimerError(Exception):
    """yeey"""
    

class Timer:
    def __init__(self):
        self._start_time = None
        self._stop_time = None
    def start(self):
        if self._start_time is not None:
            raise TimerError(f"timer is running")
            
        self._start_time = time.perf_counter()
    def stop(self):
        if self._start_time is None:
            raise TimerError(f"timer is not running")
        
        
        
        self._stop_time = time.perf_counter()
        
    def getElapsed(self):
        elapsed = self._stop_time - self._start_time
        return  f"{elapsed:0.6f}"
    def getElapsedFloat(self):
        return (self._stop_time - self._start_time)