from multipledispatch import dispatch

'''
inNumber - the number for the function to run the conjecture on
returnMode - the mode for what thing to return:
                    0 - number of iterations
                    1 - odd iterations
                    2 - even iterations
                    3 - return all three as a list : [TOTAL_ITERATIONS, ODD_ITERATIONS, EVEN_ITERATIONS]


'''
#i just learned what default arguments are
def collatzRun(inNumber, returnMode = 0, printSteps = False):
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
        
    match(returnMode):   #checks for returnMode input
        case 0: 
            return collatzIterations
        case 1:
            return oddIters
        case 2:
            return evenIters
        case 3:
            return [collatzIterations, oddIters, evenIters]
        case _:
            return collatzIterations
            
