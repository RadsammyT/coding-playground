#i just learned what default arguments are
def collatzRun(inNumber, printSteps = False):
    collatzIterations = 0
    oddIters = 0
    evenIters = 0
    
    
    
    while True:
        if inNumber % 2 == 1:
            inNumber = (3 * inNumber ) + 1 #apply 3x + 1
            collatzIterations += 1
            oddIters += 1
        
        elif inNumber % 2 == 0:
            inNumber /= 2                  # Divide inNumber by half
            collatzIterations += 1
            evenIters += 1
        
        if printSteps == 1:
            print(f"{inNumber:0.0f}")
            
        if inNumber == 1:
            break
        return [collatzIterations, oddIters, evenIters]
