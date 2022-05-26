import random


def run(len, repeatMax):
    """
    :param len: length of the list
    :param repeat: how many times the list should be shuffled
    :return: list of shuffled numbers

    randomizes the list from numbers 1 to len
    if array repeats any numbers, it will randomize the list again

    """
    list = []
    repeat = 0
    fails = 0
    for i in range(1, len + 1):
        list.append(i)

    while not repeat >= repeatMax:
        list = shuffle(list)
        if isUnique(list):
            print(list)
            repeat = repeat + 1
        else:
            fails = fails + 1
            print("fails: " + str(fails) + "\r", end="")
            

        
    
def isUnique(list):
    """
    :return: True if the list is unique, False if not
    """
    for i in range(len(list)):
        for j in range(len(list)):
            if list[i] == list[j] and i != j:
                return False
    return True

def shuffle(list):
    """
    :param list: list to be shuffled
    :return: shuffled list
    """
    for i in range(len(list)):
        list[i] = random.randint(0, len(list) - 1)
    return list
    
    
def __main__():
    run(10, 10)

#__main__()